/*
Daenerys: "(...) so I should welcome you into my service because you murdered members of your own family?"
Tyrion: "Into your service? Your Grace, we have only just met. It's too soon to know if you deserve my service."

To settle things between her and the imp, Daenerys decides to challenge Tyrion to play a 2-players game she invented back in the Dothraki sea. If Tyrion beats Daenerys in the game, she will make him his advisor, as he desires. Otherwise, she will have him executed.

This is a game with words. In this game, it is considered only the Dothraki alphabet, which consists on the first L lowercase letters of the English alphabet.

The game starts with a list of S strings s1, ..., sS. The game is played by turns, and Tyrion begins. At each turn, a player chooses one string from the list and adds a letter (from the Dothraki alphabet) to its end, in the right side. However, a player cannot add a letter to a string si (1 ≤ i ≤ S) if Fi letters were already added to that string during the game. Also, a player cannot add a letter to a string if such string, including all letters added to it during the game (but not the one the player wants to add), contains a Dothraki adjective as a substring. All Dothraki adjectives are given in the input.

If a player is not able to add any letter to any string, the player loses the game. Both Daenerys and Tyrion play optimally. Your task is to decide whether Tyrion can beat Daenerys in the game.

In the first sample, Tyrion can win by first adding the letter c to the string s1: it will contain an adjective and thus won't be chosen anymore. Daenerys is then forced to add a letter to s2. Tyrion then adds another letter to s2. Daenerys will then be unable to move, since F2 = 2 letters were already added to that string.

Input
The first line of the input contains the integers D and L (D > 0, 2 ≤ L ≤ 10), where D is the number of Dothraki adjectives. Each of the next D lines contains an adjective. The sum of the length of all adjectives does not exceed 105. The next line contains the number S (1 ≤ S ≤ 300). Each of the next S lines contains a string si and an integer Fi (1 ≤ Fi ≤ 50). The sum of the length of all strings si does not exceed 3×103.

It is guaranteed that none of these strings contains a Dothraki adjective. Also, all Dothraki adjectives and all strings si contain only letters from the Dothraki alphabet.

Output
In a single line, print Tyrion if Tyrion can beat Daenerys in the game, or Daenerys otherwise.
==============================================================================================

Input Samples       | Output Samples
--------------------------------------------------------------------------------
1 5                 | Tyrion
abc                 |
2                   |
eab 10              |
de 2                |
--------------------------------------------------------------------------------
2 4                 | Daenerys
abc                 |
acd                 |
2                   |
ab 1                |
ac 1                |
*/

pub fn run() {
    println!("BEE 1853 - Game of Trust");
}
