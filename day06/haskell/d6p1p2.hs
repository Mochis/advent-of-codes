import Data.List

packetStart :: String -> Int -> Int
packetStart [] _ = 0
packetStart ss size
    | containDups marker = 1 + packetStart (tail ss) size
    | otherwise              = size
    where
        marker = sort (take size ss)
        containDups []     = False
        containDups (x:xs) = x `elem` xs || containDups xs

main :: IO ()
main = do
    input <- readFile "../input"

    let start = packetStart input 4
    putStrLn $ "First part: " ++ show start


    let start2 = packetStart input 14
    putStrLn $ "Second part: " ++ show start2