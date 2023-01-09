import Data.List (transpose, isPrefixOf)
import Data.List.Split
import Data.Char

type Action = (Int, Int, Int)
type Crate  = Char
type Stack  = (Int, [Crate])
--
--     [D]    
-- [N] [C]      ==> [(1,"NZ"),(2,"DCM"),(3,"P")]
-- [Z] [M] [P]
--  1   2   3 
--
parseStacks :: [String] -> Int -> [Stack]
parseStacks ss max = zip [1..max] . map concat . transpose . foldr ((:) . parseCrates) [] $ ss
    where
        parseCrates [] = []
        parseCrates xss
            | isSpace c = "" : parseCrates (tail yss)
            | otherwise = [c]  : parseCrates (tail yss)
            where
                (_:c:_, yss) = splitAt 3 xss

parseAction :: String -> Action
parseAction s = (read n, read src, read dst)
    where
        [_, n, _, src, _, dst] = words s

move :: [Stack] -> [Char] -> Action -> [Stack]
move [] _ _ = []
move ((idx, stack):xs) crates action
    | idx == src = (idx, drop n stack): move xs crates action
    | idx == dst = (idx, crates ++ stack) : move xs crates action
    | otherwise  = (idx, stack) : move xs crates action
    where
        (n, src, dst) = action

pick :: [Stack] -> Action -> [Crate]
pick stacks (n, src, _) = take n (snd (stacks!!(src - 1)))

applyAction :: [Stack] -> Action -> [Stack]
applyAction stacks action = move stacks cratesToMove action
    where
        (n, src, _) = action
        cratesToMove = reverse $ pick stacks action

applyAction2 :: [Stack] -> Action -> [Stack]
applyAction2 stacks action = move stacks cratesToMove action
    where
        (n, src, _) = action
        cratesToMove = pick stacks action

cratesAtTop :: [Stack] -> String
cratesAtTop = map (head . snd)

main :: IO ()
main = do
    input <- readFile "../input"
    let [p1, p2] = splitWhen null $ lines input
    let stacks = parseStacks (init p1) (length . words . last $ p1)
    let actions = map parseAction p2

    let atTop = cratesAtTop (foldl applyAction stacks actions)
    putStrLn $ "First part: " ++ show atTop

    let atTop2 = cratesAtTop (foldl applyAction2 stacks actions)
    putStrLn $ "Second part: " ++ show atTop2