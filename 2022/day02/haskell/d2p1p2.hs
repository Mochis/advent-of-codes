data Shape = R | P | S | None
data Result = L | D | W | Non

toShape :: Char -> Shape
toShape 'A' = R
toShape 'B' = P
toShape 'C' = S
toShape 'X' = toShape 'A'
toShape 'Y' = toShape 'B'
toShape 'Z' = toShape 'C'
toShape _   = None

toResult :: Char -> Result
toResult 'X' = L
toResult 'Y' = D
toResult 'Z' = W
toResult _   = Non

parseRound :: (Char -> a) -> String -> (Shape, a)
parseRound f [h1, ' ', h2] = (toShape h1, f h2)
parseRound f xs = (None, f ' ')

pointsByShape :: Shape -> Int
pointsByShape R = 1
pointsByShape P = 2
pointsByShape S = 3
pointsByShape None = 0

win :: Int
win = 6
draw :: Int
draw = 3

evalRound :: (Shape, Shape) -> (Int, Int)
evalRound (R, P) = (pointsByShape R, win + pointsByShape P)
evalRound (R, S) = (win + pointsByShape R, pointsByShape S)
evalRound (P, R) = (win + pointsByShape P, pointsByShape R)
evalRound (S, R) = (pointsByShape S, win + pointsByShape R)
evalRound (S, P) = (win + pointsByShape S, pointsByShape P)
evalRound (P, S) = (pointsByShape P, win + pointsByShape S)
evalRound (s1, s2) = (draw + pointsByShape s1, draw + pointsByShape s2)

guessRound :: (Shape, Result) -> (Shape, Shape)
guessRound (R, W) = (R, P)
guessRound (R, L) = (R, S)
guessRound (P, W) = (P, S)
guessRound (P, L) = (P, R)
guessRound (S, W) = (S, R)
guessRound (S, L) = (S, P)
guessRound (s1, D) = (s1, s1)
guessRound (_, _) = (None, None)


main :: IO ()
main = do
    game <- readFile "../input"
    let rounds = lines game -- [r1, r2, r3] -> f r1 + f r2 + f r3
    let score1 = foldr ((+) . snd . evalRound . parseRound toShape) 0 rounds
    putStrLn $ "First part: " ++ show score1
    let score2 = foldr ((+) . snd . evalRound . guessRound . parseRound toResult) 0 rounds
    putStrLn $ "Second part: " ++ show score2
