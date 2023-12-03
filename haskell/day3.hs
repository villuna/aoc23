import Data.Char

type Position = (Int, Int)
type CharMap = [(Char, Position)]
type NumberMap = [(Int, [Position])]

main :: IO ()
main = do
    input <- readFile "../input/day3.txt"
    let nMap = numberMap input
    let cMap = charMap input

    putStrLn $ "part 1: " ++ show (part1 nMap cMap)
    putStrLn $ "part 2: " ++ show (part2 nMap cMap)

charMap :: String -> CharMap
charMap input = filter (not . (flip elem ".1234567890") . fst) (flatten ichars)
    where
        flatten :: [(Int, [(Int, Char)])] -> [(Char, Position)]
        flatten = concatMap (\(i, cs) -> map (\(j, c) -> (c, (i, j))) cs)

        ichars :: [(Int, [(Int, Char)])]
        ichars = zip [0..] $ map (zip [0..]) (lines input)


numberMap :: String -> NumberMap
numberMap input = ichars >>= (uncurry scanLine)
    where
        ichars :: [(Int, [(Int, Char)])]
        ichars = zip [0..] $ map (zip [0..]) (lines input)

        scanLine :: Int -> [(Int, Char)] -> [(Int, [Position])]
        scanLine i line = map (\(n, coords) -> (n, zip (repeat i) coords)) (scanNums line)  

        scanNums :: [(Int, Char)] -> [(Int, [Int])]
        scanNums line = snd $ (head . (dropWhile (not . null . fst))) (iterate getNextNum (line, []))

        getNextNum :: ([(Int, Char)], [(Int, [Int])]) -> ([(Int, Char)], [(Int, [Int])])
        getNextNum ([], l) = ([], l)
        getNextNum (line, list) = 
            if null stripped then 
                ([], list)
            else
                (
                    dropWhile (isDigit . snd) stripped, 
                    list ++ [(read (takeWhile isDigit (map snd stripped)), map fst (takeWhile (isDigit . snd) stripped))]
                )
                where
                    stripped = dropWhile (not . isDigit . snd) line

isAdjacent :: Position -> Position -> Bool
isAdjacent (i1, j1) (i2, j2) = abs (i1 - i2) <= 1 && abs (j1 - j2) <= 1

part1 :: NumberMap -> CharMap -> Int
part1 nMap cMap = sum numbers
    where
        numbers :: [Int]
        numbers = map fst (filter (adjacentToAnything . snd) nMap)

        adjacentToAnything :: [Position] -> Bool
        adjacentToAnything ps = any (\p -> any (isAdjacent p) (map snd cMap)) ps
    
part2 :: NumberMap -> CharMap -> Int
part2 nMap cMap = sum gearValues
    where
        gearValues :: [Int]
        gearValues = map product $ filter ((==2) . length) (map adjacentNumbers asters)

        asters :: [Position]
        asters = map snd (filter ((=='*') . fst) cMap)

        adjacentNumbers :: Position -> [Int]
        adjacentNumbers pos = map fst $ filter (any (isAdjacent pos) . snd) nMap

