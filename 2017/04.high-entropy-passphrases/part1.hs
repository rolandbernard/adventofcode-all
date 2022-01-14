
import Data.List.Split
import qualified Data.HashSet as HashSet

hasDuplicates _ [] = False
hasDuplicates v (x:xs)
    | HashSet.member x v = True
    | otherwise = hasDuplicates (HashSet.insert x v) xs

main = do
    dat <- getContents
    let list = filter (not . hasDuplicates HashSet.empty) (map (splitOn " ") (filter (not . null) (splitOn "\n" dat)))
    putStrLn ("Result: " ++ show (length list))

