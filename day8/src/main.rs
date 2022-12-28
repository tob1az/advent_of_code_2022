use ndarray::{Array, Array2, ArrayView2};

mod data;

type TreeHeight = i8;
type TreeVisibility = (TreeHeight, bool);
type VisibleTrees = Array2<TreeVisibility>;

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

fn calculate_solution(tree_heights: &str) -> usize {
    let visible_trees: VisibleTrees =
        mark_visible_tree_row(mark_visible_tree_row(parse_trees(tree_heights).view()).t());
    visible_trees.into_iter().filter(|(_, v)| *v).count()
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::TREE_HEIGHTS));
}
