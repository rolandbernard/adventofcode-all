
import Data.List.Split

main = do
    dat <- getContents
    let list = map (map read . splitOn "\t") (filter (not . null) (splitOn "\n" dat)) :: [[Int]]
    let result = sum (map (\l -> maximum [div x y | x <- l, y <- l, mod x y == 0]) list)
    putStrLn ("Result: " ++ show result)

