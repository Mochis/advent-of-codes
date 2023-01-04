import BTree

priority :: Char -> Int
priority c
    | c < 'a'   = ascii `mod` fromEnum 'A' + 26 + 1 -- 'A' = 65 ~ 1, offset = 26
    | otherwise = ascii `mod` fromEnum 'a' + 1      -- 'a' = 97
    where
        ascii = fromEnum c

parseRucksack :: String -> (Tree Char, [String])
parseRucksack s = ((toTree . fst) split, [snd split])
    where
        middle = length s `div` 2
        split  = splitAt middle s

parseBadge :: [String] -> (Tree Char, [String])
parseBadge []     = error "Invalid badge"
parseBadge (x:xs) = (toTree x, xs)

toTree :: String -> Tree Char
toTree = foldl BTree.insert nilTree

anyContained :: Tree Char -> String -> Char
anyContained _ [] = error "No element contained"
anyContained tree (x:xs)
    | contains tree x = x
    | otherwise       = anyContained tree xs

-- Find a contained element in a list of strings in O(n) * O(n * log n) time.
-- Intersect would do in O(n) * O(n^2) in an unordered string
duplicated :: (Tree Char, [String]) -> Char
duplicated (tree, xs) = head $ foldr ((:) . anyContained tree) [] xs

chunk :: Int -> [a] -> [[a]]
chunk _ [] = []
chunk n xs = y1 : chunk n y2
  where
    (y1, y2) = splitAt n xs

main :: IO ()
main = do
    input <- readFile "../input"
    
    let rucksacks = lines input
    let rucksacksPrioritySum = foldr ((+) . priority . duplicated . parseRucksack) 0 rucksacks
    putStrLn $ "First part: " ++ show rucksacksPrioritySum

    let badges = chunk 3 rucksacks
    let badgesPrioritySum = foldr ((+) . priority . duplicated . parseBadge) 0 badges
    putStrLn $ "Second part: " ++ show badgesPrioritySum