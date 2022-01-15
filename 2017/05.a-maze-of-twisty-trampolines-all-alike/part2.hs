
import Data.List.Split
import qualified Data.Sequence as Seq

run insts pc i
    | pc < 0 || pc >= Seq.length insts = i
    | otherwise = run (Seq.update pc d insts) (pc + j) (i + 1)
        where
            j = Seq.index insts pc
            d = j + if j < 3 then 1 else -1

main = do
    dat <- getContents
    let list = Seq.fromList (map read (filter (not . null) (splitOn "\n" dat)) :: [Int])
    putStrLn ("Result: " ++ show (run list 0 0))

