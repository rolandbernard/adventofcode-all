
import Data.List.Split
import qualified Data.HashSet as HashSet
import qualified Data.HashMap.Strict as HashMap

counts = foldl (\a x -> HashMap.insert x (1 + HashMap.findWithDefault 0 x a) a) HashMap.empty

hasDuplicates _ [] = False
hasDuplicates v (x:xs) = HashSet.member c v || hasDuplicates (HashSet.insert c v) xs
    where c = counts x :: HashMap.HashMap Char Int

main = do
    dat <- getContents
    let list = filter (not . hasDuplicates HashSet.empty) (map (splitOn " ") (filter (not . null) (splitOn "\n" dat)))
    putStrLn ("Result: " ++ show (length list))

