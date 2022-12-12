import System.IO

main = do
    input <- readFile "../input"
    let elvesCals = lines input
    let maxCals = bestElf elvesCals 0 0
    putStrLn $ "The max is: " ++ show maxCals 

-- Recursivelly accumulates partial calories and the best work
bestElf :: [String] -> Int -> Int -> Int
bestElf [] _ best = best
bestElf ("":xs) _ best = bestElf xs 0 best -- reset accumulated
bestElf (x:xs) partial best = 
    let elfCals = partial + strToInt x
    in bestElf xs elfCals (max elfCals best)

strToInt :: String -> Int
strToInt s = read s :: Int

-- ["1", "3", "\n", "4", "2"]
-- => "1" : "3" : "" : ["4, "2"], partial = 4, best = 4, next call -> bestElf ["4, "2"] 0 4
-- ...
-- => "1" : "3" : "" : "4" : ["2"], x = 4, partial = 0, best = 4, next call -> bestElf ["2"] (4 + 0) 6
-- => "1" : "3" : "" : "4" : "2" : [], x = 2, partial = 4, best = 4 + 2, next call -> bestElf [] 4 6



