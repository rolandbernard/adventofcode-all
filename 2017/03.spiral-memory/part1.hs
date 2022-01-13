
import Data.List.Split

findLayer x l o =
    if x <= 8 * l + o then
        (l, x - o)
    else
        findLayer x (l + 1) (8 * l + o)

toPosition (l, o)
    | o <= 2 * l = (l, o - l)
    | o <= 4 * l = (3 * l - o, l)
    | o <= 6 * l = (-l, 5 * l - o)
    | otherwise = (-l, o - 7 * l)

findLocaition x =
    if x == 1 then
        (0, 0)
    else
        toPosition (findLayer x 1 1)

main = do
    dat <- getContents
    let field = read dat :: Int
    let (x, y) = findLocaition field
    putStrLn ("Result: " ++ show (abs x + abs y))

