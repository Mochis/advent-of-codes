import BTree

priority :: Char -> Int
priority c
    | c < 'a' = ascii `mod` 65 + 26 + 1 -- 'A' = 65 ~ 1, offset = 26
    | otherwise = ascii `mod` 97 + 1 -- 'a' = 97
    where
        ascii = fromEnum c

parseRucksack :: String -> (Tree Char, String)
parseRucksack s = ((toTreeChar nilTree . fst) split, snd split)
    where
        middle = length s `div` 2
        split = splitAt middle s

parseRucksack2 :: [String] -> (Tree Char, [String])
parseRucksack2 [] = error "Invalid badge"
parseRucksack2 (x:xs) = (toTreeChar nilTree x, xs)

toTreeChar :: Tree Char -> String -> Tree Char
toTreeChar = foldl BTree.insert

duplicated :: (Tree Char, String) -> Char
duplicated (_, []) = ' '
duplicated (tree, x:xs)
    | contains tree x = x
    | otherwise = duplicated (tree, xs)

duplicated2 :: (Tree Char, [String]) -> Char
duplicated2 (_, []) = error "No duplicated element"
duplicated2 (tree, x:xs) 
    | duplicatedChar /= ' ' = duplicatedChar
    | otherwise = duplicated2 (tree, xs)
    where
        duplicatedChar = duplicated (tree, x)

chunk :: Int -> [a] -> [[a]]
chunk _ [] = []
chunk n xs = y1 : chunk n y2
  where
    (y1, y2) = splitAt n xs

main :: IO ()
main = do
    input <- readFile "../input"
    
    let rucksacks = lines input
    let rucksackSum1 = foldr ((+) . priority . duplicated . parseRucksack) 0 rucksacks
    putStrLn $ "First part: " ++ show rucksackSum1

    let rucksacks3 = chunk 3 rucksacks
    let rucksackSum2 = foldr ((+) . priority . duplicated2 . parseRucksack2) 0 rucksacks3
    putStrLn $ "Second part: " ++ show rucksackSum2