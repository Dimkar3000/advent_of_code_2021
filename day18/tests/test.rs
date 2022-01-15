use snailfish::*;

#[test]
fn simple() {
    let snail = SnailFish::single("[1,[[56,7,[8]]],2,[1,2]]");
    assert_eq!(snail.to_string(), "[1,[[56,7,[8]]],2,[1,2]]")
}

#[test]
fn multi() {
    let snail = SnailFish::multiple_line("[1,2]\n[[1,2],3]\n[8,9]");
    assert_eq!(snail.to_string(), "[[[1,2],[[1,2],3]],[8,9]]")
}

#[test]
fn node_split1() {
    let mut snail = SnailFish::single("[1,10]");
    assert!(snail.split());
    assert_eq!(snail.to_string(), "[1,[5,5]]")
}

#[test]
fn node_split2() {
    let mut snail = SnailFish::single("[[[[0,7],4],[15,[0,13]]],[1,1]]");
    assert!(snail.split());
    assert_eq!(snail.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    assert!(snail.split());
    assert_eq!(snail.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    assert!(!snail.split());
}

#[test]
fn addition() {
    let snail = SnailFish::multiple_line("[1,2]\n[[3,4],5]");
    assert_eq!(snail.to_string(), "[[1,2],[[3,4],5]]");
}

#[test]
fn explosion() {
    let data = [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ),
        (
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
    ];
    let mut nodes: Vec<(SnailFish, String)> = data
        .iter()
        .map(|x| (SnailFish::single(x.0), x.1.to_string()))
        .collect();
    for (snail, result) in nodes.iter_mut() {
        snail.explode();
        assert_eq!(snail.to_string(), *result);
    }
}

#[test]
fn split() {
    let data = [
        ("[10,10]", "[[5,5],10]"),
        ("[11,10]", "[[5,6],10]"),
        ("[12,10]", "[[6,6],10]"),
        ("[9,10]", "[9,[5,5]]"),
        ("[5,10]", "[5,[5,5]]"),
    ];
    let mut nodes: Vec<(SnailFish, String)> = data
        .iter()
        .map(|x| (SnailFish::single(x.0), x.1.to_string()))
        .collect();
    for (snail, result) in nodes.iter_mut() {
        snail.split();
        assert_eq!(snail.to_string(), *result);
    }
}
#[test]
fn explode_test() {
    let mut snail = SnailFish::single("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    assert!(snail.explode());
    assert_eq!(snail.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn manual_procedure() {
    let mut snail = SnailFish::raw_multiple_line("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]");
    assert_eq!(snail.to_string(), "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");

    snail.explode();
    assert_eq!(snail.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
    snail.explode();
    assert_eq!(snail.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
    snail.split();
    assert_eq!(snail.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    snail.split();
    assert_eq!(snail.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    snail.explode();
    assert_eq!(snail.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn auto_procedure() {
    let mut snail = SnailFish::multiple_line("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]");
    snail.reduce();
    assert_eq!(snail.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn single_line_explode() {
    let mut snail =
        SnailFish::single("[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[0,[11,3]],[[6,3],[8,8]]]]]");
    snail.explode();
    assert_eq!(
        snail.to_string(),
        "[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[11,0],[[9,3],[8,8]]]]]"
    );
}

#[test]
fn more1() {
    let mut snail = SnailFish::multiple_line(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
    );
    snail.reduce();
    assert_eq!(
        snail.to_string(),
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
    );
}
