namespace Fs
open Framework
open Utils

module Day2 = 
    exception ParseError of string

    type Hand = { red: int; green: int; blue: int }
    
    let maxRed = 12
    let maxGreen = 13
    let maxBlue = 14

    let applyHand hand input =
        let splits = split " " input
        let number = int splits[0]

        match splits[1] with
        | "red" -> { hand with red = number }
        | "green" -> { hand with green = number }
        | "blue" -> { hand with blue = number }
        | _ -> raise (ParseError("invalid colour: " + splits[1]))

    let parseHand input =
        let emptyHand = { red = 0; green = 0; blue = 0 }

        input
        |> split ", "
        |> List.fold applyHand emptyHand

    let parseGame input =
        (split ": " input)[1] |> split "; " |> List.map parseHand

    let parseInput input =
        input
        |> split "\n"
        |> List.filter (fun s -> s <> "")
        |> List.map parseGame

    let isLegal (game: list<Hand>) =
        game
        |> List.map (fun hand -> hand.red <= maxRed && hand.green <= maxGreen && hand.blue <= maxBlue)
        |> List.fold (&&) true

    let part1 games =
        games 
        |> List.indexed
        |> List.filter (snd >> isLegal)
        |> List.map (fst >> ((+) 1))
        |> List.sum

    let part2 games =
        games
        |> List.map (List.reduce (fun acc hand -> { red = max acc.red hand.red; 
                                                    green = max acc.green hand.green; 
                                                    blue = max acc.blue hand.blue }))
        |> List.map (fun hand -> hand.red * hand.blue * hand.green)
        |> List.sum

    let day2 (input: string) =
        let games = parseInput input
        printfn "part 1: %d" (part1 games)
        printfn "part 2: %d" (part2 games)

type Day2() =
    interface IDay with
        member this.Run (input: string) = Day2.day2 input
