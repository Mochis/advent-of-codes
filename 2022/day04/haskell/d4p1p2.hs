import Data.List.Split (splitOn)

type Range = (Int, Int)

parseRange :: String -> Range
parseRange = tuplify2 . map read . splitOn "-"

parseRanges :: String -> (Range, Range)
parseRanges = tuplify2 . map parseRange . splitOn ","

tuplify2 :: [a] -> (a, a)
tuplify2 [x, y] = (x, y)
tuplify2 _ = error "Invalid list"

fullyOverlaps :: Range -> Range -> Bool
fullyOverlaps (a, b) (c, d) = (a <= c && b >= d) || (a >= c && b <= d)

overlaps :: Range -> Range -> Bool
overlaps (a, b) (c, d) = a <= d && b >= c

main :: IO ()
main = do
    input <- readFile "../input"
    let pairs = lines input
    let ranges = map parseRanges pairs

    let fullyOverlapsSum = foldr ((+) . fromEnum . uncurry fullyOverlaps) 0 ranges
    putStrLn $ "First part: " ++ show fullyOverlapsSum

    let overlapsSum = foldr ((+) . fromEnum . uncurry overlaps) 0 ranges
    putStrLn $ "Second part: " ++ show overlapsSum