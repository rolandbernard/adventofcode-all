
import Data.List.Split

main = do
    dat <- getContents
    let list = map (map read . splitOn "\t") (filter (not . null) (splitOn "\n" dat)) :: [[Int]]
    let result = sum (map (\x -> maximum x - minimum x) list)
    putStrLn ("Result: " ++ show result)

