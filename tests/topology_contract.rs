//! Exact, hand-derived fixtures built only through the consumer-facing API.
use std::collections::{BTreeMap, BTreeSet, HashSet};

use generalized_map::{DartIndex, NGMap};

type Darts = BTreeSet<usize>;

fn set(darts: impl IntoIterator<Item = DartIndex>) -> Darts {
    darts.into_iter().map(|d| *d).collect()
}

// Independent reachability oracle: no crate iterators or orbit-sequence helpers.
fn reachable<const N: usize>(map: &NGMap<N>, seed: DartIndex, axes: &[usize]) -> Darts {
    let mut seen = Darts::new();
    let mut pending = vec![seed];
    while let Some(d) = pending.pop() {
        if seen.insert(*d) {
            for &i in axes {
                pending.push(map.get_alpha(&d, i));
            }
        }
    }
    seen
}

fn cell<const N: usize>(map: &NGMap<N>, d: DartIndex, i: usize, active: usize) -> Darts {
    let axes: Vec<_> = (0..active).filter(|&j| j != i).collect();
    reachable(map, d, &axes)
}

fn cells<const N: usize>(map: &NGMap<N>, i: usize, active: usize) -> BTreeSet<Darts> {
    map.iter_all().map(|d| cell(map, d, i, active)).collect()
}

fn counts<const N: usize>(map: &NGMap<N>, expected: &[usize]) {
    for (i, &n) in expected.iter().enumerate() {
        assert_eq!(map.number_cells(i), n, "dimension {i}");
    }
}

fn verify<const N: usize>(map: &NGMap<N>, active: usize) {
    assert_eq!(map.number_alpha_nonfree(), active);
    let darts: Vec<_> = map.iter_all().collect();
    let all = set(darts.iter().copied());
    assert!(!all.contains(&0), "null sentinel is not a live dart");
    for &d in &darts {
        for i in 0..active {
            let a = map.get_alpha(&d, i);
            assert!(all.contains(&*a));
            assert_eq!(map.get_alpha(&a, i), d, "alpha involution");
            assert_eq!(map.is_free(&d, i), a == d);
            assert_eq!(map.get_alpha_if_exists(&d, i), (a != d).then_some(a));
            for j in i + 2..active {
                let ij = map.get_alpha(&a, j);
                let iji = map.get_alpha(&ij, i);
                assert_eq!(map.get_alpha(&iji, j), d, "(alpha_i alpha_j)^2");
                assert_eq!(ij, map.get_alpha(&map.get_alpha(&d, j), i));
            }
        }
    }
    for i in 0..active {
        let expected = cells(map, i, active);
        let representatives: Vec<_> = map.iter_dart_per_cell(i).collect();
        assert_eq!(representatives.len(), expected.len());
        let actual: BTreeSet<_> = representatives
            .iter()
            .map(|&d| cell(map, d, i, active))
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(expected.iter().map(Darts::len).sum::<usize>(), all.len());
        assert_eq!(expected.iter().flatten().copied().collect::<Darts>(), all);
        for &d in &darts {
            let source = cell(map, d, i, active);
            assert_eq!(set(map.iter_darts_in_cell(d, i)), source);
            for &e in &darts {
                assert_eq!(map.darts_in_same_cell(&d, &e, i), source.contains(&*e));
                if source.contains(&*e) {
                    assert_eq!(cell(map, e, i, active), source);
                }
            }
            for j in 0..active {
                if i == j {
                    continue;
                }
                // API arguments are TARGET dimension, SOURCE dimension.
                let incident: Vec<_> = map.iter_incident(d, j, i).collect();
                let actual: BTreeSet<_> =
                    incident.iter().map(|&e| cell(map, e, j, active)).collect();
                let expected: BTreeSet<_> = cells(map, j, active)
                    .into_iter()
                    .filter(|c| !c.is_disjoint(&source))
                    .collect();
                assert_eq!(
                    incident.len(),
                    actual.len(),
                    "one representative per incident cell"
                );
                assert_eq!(actual, expected);
            }
        }
    }
    let axes: Vec<_> = (0..active).collect();
    let expected: BTreeSet<_> = darts.iter().map(|&d| reachable(map, d, &axes)).collect();
    let reps: Vec<_> = map.iter_dart_per_component().collect();
    assert_eq!(reps.len(), expected.len());
    assert_eq!(
        reps.iter()
            .map(|&d| reachable(map, d, &axes))
            .collect::<BTreeSet<_>>(),
        expected
    );
    assert_eq!(map.number_connected_components(), expected.len());
    for &d in &darts {
        assert_eq!(
            set(map.iter_darts_in_component(d)),
            reachable(map, d, &axes)
        );
    }
}

fn verify_adjacency<const N: usize>(map: &NGMap<N>, active: usize) {
    for d in map.iter_all() {
        for i in 0..active {
            let source = cell(map, d, i, active);
            // Alpha-i adjacency, not arbitrary incidence through a lower cell.
            let expected: BTreeSet<_> = source
                .iter()
                .filter_map(|&e| {
                    let a = map.get_alpha(&e.into(), i);
                    (*a != e).then(|| cell(map, a, i, active))
                })
                .collect();
            let neighbors: Vec<_> = map.iter_adjacent(d, i).collect();
            let actual: BTreeSet<_> = neighbors.iter().map(|&e| cell(map, e, i, active)).collect();
            assert_eq!(neighbors.len(), actual.len());
            assert_eq!(actual, expected, "distinct adjacent {i}-cells at {d}");
        }
    }
}

// A local flag walk, not a chain boundary implementation. Each returned dart
// chooses the source endpoint of one occurrence of an edge in this face.
fn face_walk<const N: usize>(map: &NGMap<N>, start: DartIndex) -> Vec<DartIndex> {
    let mut walk = Vec::new();
    let mut d = start;
    loop {
        assert!(!walk.contains(&d));
        walk.push(d);
        d = map.get_alpha(&map.get_alpha(&d, 0), 1);
        if d == start {
            return walk;
        }
        assert!(walk.len() <= map.iter_all().count());
    }
}

#[test]
fn single_edge_two_endpoints() {
    let mut map = NGMap::<2>::new(2);
    let (a, b) = map.create_edge();
    assert_eq!(set(map.iter_all()), set([a, b]));
    counts(&map, &[2, 1]);
    assert_eq!(map.get_alpha(&a, 0), b);
    assert_eq!(map.get_alpha(&b, 0), a);
    assert!(map.is_free(&a, 1) && map.is_free(&b, 1));
    assert_eq!(set(map.iter_incident(a, 0, 1)), set([a, b]));
    assert_eq!(cell(&map, a, 0, 2), set([a]));
    assert_eq!(cell(&map, a, 1, 2), set([a, b]));
    verify(&map, 2);
}

fn square<const N: usize>(active: usize) -> (NGMap<N>, DartIndex) {
    let mut map = NGMap::<N>::new(active);
    let ring = map.create_ring(4);
    (map, ring[0].0)
}

#[test]
fn square_face_and_local_orientation() {
    let (map, start) = square::<3>(3);
    counts(&map, &[4, 4, 1]);
    assert_eq!(map.iter_all().count(), 8);
    assert_eq!(cell(&map, start, 2, 3).len(), 8);
    let walk = face_walk(&map, start);
    assert_eq!(walk.len(), 4);
    let edges: BTreeSet<_> = walk.iter().map(|&d| cell(&map, d, 1, 3)).collect();
    assert_eq!(edges.len(), 4);
    let reversed = face_walk(&map, map.get_alpha(&start, 0));
    let mut expected = vec![map.get_alpha(&walk[0], 0)];
    expected.extend(walk[1..].iter().rev().map(|d| map.get_alpha(d, 0)));
    assert_eq!(
        reversed, expected,
        "reversing local orientation reverses edge order and endpoints"
    );
    for (k, &d) in walk.iter().enumerate() {
        assert!(map.is_free(&d, 2));
        assert_eq!(cell(&map, d, 1, 3).len(), 2);
        assert_eq!(map.iter_incident(d, 0, 1).count(), 2);
        let next = walk[(k + 1) % 4];
        assert_eq!(
            cell(&map, map.get_alpha(&d, 0), 0, 3),
            cell(&map, next, 0, 3)
        );
    }
    assert_eq!(map.iter_incident(start, 1, 2).count(), 4);
    verify(&map, 3);
}

#[test]
fn arbitrary_orbits_are_independent_of_generator_order() {
    let (map, _, _) = joined_squares::<3>(3, false);
    for axes in [vec![], vec![0], vec![2, 0], vec![1, 2], vec![0, 1, 2]] {
        let reverse: Vec<_> = axes.iter().rev().copied().collect();
        for d in map.iter_all() {
            let expected = reachable(&map, d, &axes);
            assert_eq!(set(map.iter_orbit(d, &axes)), expected);
            assert_eq!(set(map.iter_orbit(d, &reverse)), expected);
            for &e in &expected {
                assert_eq!(reachable(&map, e.into(), &axes), expected);
            }
        }
    }
}

fn joined_squares<const N: usize>(
    active: usize,
    reverse: bool,
) -> (NGMap<N>, DartIndex, DartIndex) {
    let (mut map, a) = square::<N>(active);
    let ring = map.create_ring(4);
    let b = if reverse {
        map.get_alpha(&ring[0].0, 0)
    } else {
        ring[0].0
    };
    assert!(map.is_sewable(&a, &b, 2));
    assert!(map.sew(&a, &b, 2));
    (map, a, b)
}

#[test]
fn two_squares_share_exactly_one_edge() {
    for reverse in [false, true] {
        let (map, a, b) = joined_squares::<3>(3, reverse);
        counts(&map, &[6, 7, 2]);
        assert_eq!(map.iter_all().count(), 16);
        let shared = cell(&map, a, 1, 3);
        assert_eq!(shared, cell(&map, b, 1, 3));
        assert_eq!(shared.len(), 4);
        assert_ne!(cell(&map, a, 2, 3), cell(&map, b, 2, 3));
        assert_eq!(map.get_alpha(&a, 2), b);
        assert_eq!(
            map.get_alpha(&map.get_alpha(&a, 0), 2),
            map.get_alpha(&b, 0)
        );
        assert_eq!(map.iter_incident(a, 2, 1).count(), 2);
        let external: Vec<_> = map
            .iter_dart_per_cell(1)
            .filter(|d| map.is_free(d, 2))
            .collect();
        assert_eq!(external.len(), 6);
        assert_eq!(
            external
                .iter()
                .map(|&d| cell(&map, d, 1, 3))
                .collect::<BTreeSet<_>>()
                .len(),
            6
        );
        for e in external {
            assert_eq!(map.iter_incident(e, 2, 1).count(), 1);
        }
        // Opposite boundary directions along the common edge can be selected
        // entirely through alpha0 and alpha2; no global orientation is stored.
        let wa = face_walk(&map, a);
        let wb = face_walk(&map, map.get_alpha(&b, 0));
        let ea: BTreeSet<_> = wa.iter().map(|&d| cell(&map, d, 1, 3)).collect();
        let eb: BTreeSet<_> = wb.iter().map(|&d| cell(&map, d, 1, 3)).collect();
        assert_eq!(
            ea.intersection(&eb).cloned().collect::<Vec<_>>(),
            vec![shared]
        );
        assert_eq!(
            cell(&map, a, 0, 3),
            cell(&map, map.get_alpha(&wb[0], 0), 0, 3)
        );
        verify(&map, 3);
    }
}

#[test]
fn paths_cycles_and_disconnected_components() {
    let mut path = NGMap::<2>::new(2);
    path.create_chain(3);
    counts(&path, &[4, 3]);
    assert_eq!(path.iter_all().filter(|d| path.is_free(d, 1)).count(), 2);
    verify(&path, 2);
    let mut cycle = NGMap::<2>::new(2);
    cycle.create_ring(4);
    counts(&cycle, &[4, 4]);
    assert!(cycle.iter_all().all(|d| !cycle.is_free(&d, 1)));
    verify(&cycle, 2);
    let mut disconnected = NGMap::<3>::new(3);
    disconnected.create_ring(4);
    disconnected.create_ring(4);
    counts(&disconnected, &[8, 8, 2]);
    assert_eq!(disconnected.number_connected_components(), 2);
    verify(&disconnected, 3);
}

#[test]
fn isolated_dart_is_free_on_every_active_alpha() {
    let mut map = NGMap::<3>::new(3);
    let d = map.create_dart();
    counts(&map, &[1, 1, 1]);
    for i in 0..3 {
        assert_eq!(map.get_alpha_if_exists(&d, i), None);
    }
    verify(&map, 3);
}

#[test]
fn spare_capacity_does_not_change_active_cell_orbits() {
    let (map, _) = square::<4>(3);
    counts(&map, &[4, 4, 1]);
    assert!(map.iter_all().all(|d| *map.get_alpha(&d, 3) == 0));
    verify(&map, 3);
    let (joined, _, _) = joined_squares::<5>(3, false);
    counts(&joined, &[6, 7, 2]);
    verify(&joined, 3);
    let mut path = NGMap::<4>::new(2);
    path.create_chain(3);
    counts(&path, &[4, 3]);
    verify(&path, 2);
}

#[test]
fn changing_active_dimension_preserves_lower_cells() {
    let mut map = NGMap::<4>::new(2);
    let ring = map.create_ring(4);
    let before = cells(&map, 0, 2);
    map.change_dim(3);
    counts(&map, &[4, 4, 1]);
    assert!(map.iter_all().all(|d| map.is_free(&d, 2)));
    assert_eq!(cells(&map, 0, 3), before);
    verify(&map, 3);
    map.change_dim(2);
    assert_eq!(cells(&map, 0, 2), before);
    assert_eq!(face_walk(&map, ring[0].0).len(), 4);
    verify(&map, 2);
}

#[test]
fn relabeling_preserves_all_alpha_and_cell_membership() {
    let (original, _) = square::<3>(3);
    let source: Vec<_> = original.iter_all().collect();
    let mut relabeled = NGMap::<3>::new(3);
    // Allocate in a fixed non-geometric order, then sew corresponding pairs.
    let order = [3, 0, 6, 1, 7, 4, 2, 5];
    let mut labels = BTreeMap::new();
    for k in order {
        labels.insert(*source[k], relabeled.create_dart());
    }
    for &d in &source {
        for i in 0..3 {
            let a = original.get_alpha(&d, i);
            if *d < *a {
                assert!(relabeled.sew(&labels[&*d], &labels[&*a], i));
            }
        }
    }
    for &d in &source {
        for i in 0..3 {
            assert_eq!(
                relabeled.get_alpha(&labels[&*d], i),
                labels[&*original.get_alpha(&d, i)]
            );
            let expected: Darts = cell(&original, d, i, 3)
                .iter()
                .map(|k| *labels[k])
                .collect();
            assert_eq!(cell(&relabeled, labels[&*d], i, 3), expected);
        }
    }
    counts(&relabeled, &[4, 4, 1]);
    verify(&relabeled, 3);
}

#[test]
fn loop_retains_two_endpoint_occurrences_in_darts() {
    let mut map = NGMap::<2>::new(2);
    let ring = map.create_ring(1);
    let a = ring[0].0;
    let b = map.get_alpha(&a, 0);
    counts(&map, &[1, 1]);
    assert_ne!(a, b);
    assert_eq!(cell(&map, a, 0, 2), cell(&map, b, 0, 2));
    assert_eq!(
        map.iter_incident(a, 0, 1).count(),
        1,
        "incidence iterator deduplicates cells"
    );
    assert_eq!(
        cell(&map, a, 1, 2).len(),
        2,
        "two endpoint flags remain available"
    );
    verify(&map, 2);
}

#[test]
fn folded_square_retains_repeated_face_edge_occurrences() {
    let (mut map, start) = square::<3>(3);
    let walk = face_walk(&map, start);
    assert!(map.sew(&walk[0], &walk[2], 2));
    counts(&map, &[2, 3, 1]);
    assert_eq!(map.iter_incident(start, 1, 2).count(), 3);
    let occurrences = face_walk(&map, start);
    assert_eq!(occurrences.len(), 4);
    assert_eq!(
        occurrences
            .iter()
            .filter(|&&d| cell(&map, d, 1, 3) == cell(&map, start, 1, 3))
            .count(),
        2
    );
    verify(&map, 3);
}

#[test]
fn adjacency_skips_free_links_and_preserves_nonfree_self_incidence() {
    let (map, _) = square::<3>(3);
    verify_adjacency(&map, 3);
    let (map, _, _) = joined_squares::<3>(3, false);
    verify_adjacency(&map, 3);
    let mut loop_map = NGMap::<2>::new(2);
    loop_map.create_ring(1);
    verify_adjacency(&loop_map, 2);
}

#[test]
fn sewing_rejects_an_occupied_target_without_changing_topology() {
    let mut map = NGMap::<3>::new(3);
    let (a, b) = map.create_edge();
    let c = map.create_dart();
    let before = map.to_json();
    let sewable = map.is_sewable(&c, &b, 0);
    let sewn = map.sew(&c, &b, 0);
    assert!(
        !sewable && !sewn,
        "is_sewable={sewable}, sew={sewn}, alpha0(alpha0(a))={} (expected {a})",
        map.get_alpha(&map.get_alpha(&a, 0), 0)
    );
    assert_eq!(map.to_json(), before);
    assert_eq!(map.get_alpha(&a, 0), b);
    verify(&map, 3);
}

#[test]
fn live_iterator_marks_survive_non_lifo_drop() {
    let (map, start) = square::<3>(3);
    let first = map.iter_orbit(start, &[0, 1]);
    let second = map.iter_orbit(start, &[0, 1]);
    drop(first);
    let third = map.iter_orbit(start, &[0, 1]);
    drop(third);
    let result: Vec<_> = second.take(9).collect();
    assert_eq!(result.len(), 8);
    assert_eq!(result.iter().copied().collect::<HashSet<_>>().len(), 8);
}
