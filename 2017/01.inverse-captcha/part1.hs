
import Data.Char
import Data.List

equalElems [a] b
    | a == b = [a]
    | otherwise = []
equalElems (x:y:xs) b
    | x == y = x : equalElems (y:xs) b
    | otherwise = equalElems (y:xs) b

main = do
    dat <- getContents
    let list = map digitToInt (filter isDigit dat)
    let equl = equalElems list (head list)
    putStrLn ("Result: " ++ show (sum equl))

