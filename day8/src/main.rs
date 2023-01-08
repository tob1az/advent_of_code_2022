use ndarray::{Array, Array2, ArrayView2};

mod data;

type TreeHeight = i8;
type TreeVisibility = (TreeHeight, bool);
type VisibleTrees = Array2<TreeVisibility>;

struct TreeHeightIter<'a> {
    trees: &'a VisibleTrees,
    x_step: i32,
    y_step: i32,
    x: i32,
    y: i32,
}

impl<'a> Iterator for TreeHeightIter<'a> {
    type Item = &'a TreeHeight;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.x + self.x_step, self.y + self.y_step);
        if x < 0 || y < 0 {
            None
        } else {
            self.x = x;
            self.y = y;
            self.trees.get((x as usize, y as usize)).map(|(h, _)| h)
        }
    }
}

fn update_tree_visibility(
    max_height: &mut TreeHeight,
    visibility: &TreeVisibility,
) -> Option<TreeVisibility> {
    let &(height, visible) = visibility;
    if height > *max_height {
        *max_height = height;
        Some((height, true))
    } else {
        Some((height, visible))
    }
}

fn mark_visible_tree_row(trees: ArrayView2<'_, TreeVisibility>) -> VisibleTrees {
    let row_count = trees.nrows();
    let column_count = trees.ncols();
    Array::from_iter(trees.rows().into_iter().flat_map(|lane| {
        const MIN_HEIGHT: TreeHeight = -1;
        // test visibility from both sides of the row
        lane.into_iter()
            .scan(MIN_HEIGHT, update_tree_visibility)
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .scan(MIN_HEIGHT, update_tree_visibility)
            .collect::<Vec<_>>()
    }))
    .into_shape((row_count, column_count))
    .unwrap()
}

fn parse_trees(tree_heights: &str) -> VisibleTrees {
    let rows = tree_heights.lines().count();
    let columns = tree_heights.lines().next().unwrap().len();
    Array::from_shape_vec(
        (rows, columns),
        tree_heights
            .chars()
            .filter_map(|c| {
                if c == '\n' {
                    None
                } else {
                    Some((
                        c.to_digit(10).expect("height is a digit") as TreeHeight,
                        false,
                    ))
                }
            })
            .collect::<Vec<TreeVisibility>>(),
    )
    .expect("Correct rectangle of tree heights")
}

fn find_highest_scenic_score(trees: VisibleTrees) -> usize {
    trees
        .indexed_iter()
        .map(|((x, y), &(h, _))| calculate_scenic_score(&trees, x, y, h))
        .max()
        .expect("at least one tree")
}

fn calculate_scenic_score(
    trees: &VisibleTrees,
    column: usize,
    row: usize,
    height: TreeHeight,
) -> usize {
    if column == 0 || column == trees.ncols() - 1 || row == 0 || row == trees.nrows() - 1 {
        return 0;
    }
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(|(dx, dy)| {
            TreeHeightIter {
                trees,
                x: column as i32,
                y: row as i32,
                x_step: dx,
                y_step: dy,
            }
            .scan(false, |found_occlusion, &h| {
                if *found_occlusion {
                    None
                } else {
                    *found_occlusion = h >= height;
                    Some(h)
                }
            })
            .count()
        })
        .product()
}

fn calculate_solution(tree_heights: &str) -> (usize, usize) {
    let visible_trees: VisibleTrees =
        mark_visible_tree_row(mark_visible_tree_row(parse_trees(tree_heights).view()).t());
    let observable_tree_count = visible_trees.iter().filter(|(_, v)| *v).count();
    let highest_scenic_score = find_highest_scenic_score(visible_trees);
    (observable_tree_count, highest_scenic_score)
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::TREE_HEIGHTS));
}
