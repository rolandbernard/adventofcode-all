
import Data.Char
import Data.List

equalElems list i n ac
    | i == n = ac
    | otherwise = equalElems list (i + 1) n ac'
    where
        a = list !! i
        b = list !! ((n `div` 2 + i) `mod` n)
        ac' = if a == b then a:ac else ac

main = do
    dat <- getContents
    let list = map digitToInt (filter isDigit dat)
    let equl = equalElems list 0 (length list) []
    putStrLn ("Result: " ++ show (sum equl))

