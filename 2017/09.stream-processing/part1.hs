
import Debug.Trace

parseGarbage _ "" = (0, "")
parseGarbage i ('<':xs) = (0, skipGarbage xs)
parseGarbage i ('{':xs) = (v1 + i, r1)
    where (v1, r1) = parseElems (i + 1) xs

parseElems i s =
    case s of
        ('}':xs) -> (0, xs)
        (',':xs) ->
            let (v1, r1) = parseGarbage i xs in
            let (v2, r2) = parseElems i r1 in
            (v1 + v2, r2)
        xs ->
            let (v1, r1) = parseGarbage i xs in
            let (v2, r2) = parseElems i r1 in
            (v1 + v2, r2)

skipGarbage ('>':xs) = xs
skipGarbage ('!':_:xs) = skipGarbage xs
skipGarbage (x:xs) = skipGarbage xs

main = do
    dat <- getContents
    let (score, _) = parseGarbage 1 dat
    putStrLn ("Result: " ++ show score)

