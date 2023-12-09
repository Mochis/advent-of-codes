module BTree (
    Tree,
    insert,
    contains,
    treeOf,
    nilTree
) where

data Tree a = Empty | Node a (Tree a) (Tree a)

insert :: (Ord a) => Tree a -> a -> Tree a
insert Empty x = Node x Empty Empty
insert (Node a left right) x
    | x == a = Node a left right
    | x < a = Node a (insert left x) right
    | otherwise = Node a left (insert right x)

contains :: (Eq a) => Tree a -> a -> Bool
contains Empty _ = False
contains (Node a left right) x
    | x == a = True
    | otherwise = contains left x || contains right x

treeOf :: a -> Tree a
treeOf x = Node x Empty Empty

nilTree :: Tree a
nilTree = Empty
