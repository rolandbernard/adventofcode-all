
import qualified Data.HashMap.Strict as HashMap

findLayer x l o =
    if x <= 8 * l + o then
        (l, x - o)
    else
        findLayer x (l + 1) (8 * l + o)

toPosition (l, o)
    | o <= 2 * l = (l, o - l)
    | o <= 4 * l = (3 * l - o, l)
    | o <= 6 * l = (-l, 5 * l - o)
    | otherwise = (o - 7 * l, -l)

fillGrid g i m =
    if s > m then s else fillGrid (HashMap.insert (x, y) s g) (i + 1) m
    where
        (x, y) = toPosition (findLayer i 1 1)
        s = sum [HashMap.findWithDefault 0 (x + dx, y + dy) g | dx <- [-1, 0, 1], dy <- [-1, 0, 1]]

main = do
    dat <- getContents
    let field = read dat :: Int
    let res = fillGrid (HashMap.singleton (0, 0) 1) (2 :: Int) field 
    putStrLn ("Result: " ++ show res)

