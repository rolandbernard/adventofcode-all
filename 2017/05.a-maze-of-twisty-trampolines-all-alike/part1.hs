
import Data.List.Split
import qualified Data.Sequence as Seq

run insts pc i
    | pc < 0 || pc >= Seq.length insts = i
    | otherwise = run (Seq.update pc (j + 1) insts) (pc + j) (i + 1)
        where j = Seq.index insts pc

main = do
    dat <- getContents
    let list = Seq.fromList (map read (filter (not . null) (splitOn "\n" dat)) :: [Int])
    putStrLn ("Result: " ++ show (run list 0 0))

