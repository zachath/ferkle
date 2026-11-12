# Spec Delta

## Purpose

Defines the terminal interface: how dice and scores are displayed, how the player enters input, and what feedback is shown for each game event.

## ADDED Requirements

### Requirement: Dice are displayed with position labels
The system SHALL show each rolled die's face value alongside its 1-based position number so the player can reference positions in their selection.

#### Scenario: Dice display on roll
- **WHEN** dice are rolled
- **THEN** output shows face values on one line and position numbers (1–6) on the line below, e.g. "[ 1  3  5  1  2  6 ]" / "  1  2  3  4  5  6"

### Requirement: Player selects dice by typing space-separated positions
The system SHALL accept a line of space-separated integers (1–6) as the player's dice selection and re-prompt on invalid input without discarding the turn.

#### Scenario: Valid position input
- **WHEN** the player types "1 3 5" and all referenced dice form a scoring combination
- **THEN** those dice are accepted and the player is asked to bank or roll again

#### Scenario: Out-of-range or non-integer input
- **WHEN** the player enters a position outside 1–N (where N is the number of rolled dice) or non-numeric text
- **THEN** an error line is printed and the selection prompt repeats

### Requirement: Scores and turn context are shown before each prompt
The system SHALL display both players' total scores and the current turn's accumulated score before prompting the player for input.

#### Scenario: Status display before selection prompt
- **WHEN** the player is about to select dice
- **THEN** output shows: player total, AI total, and accumulated turn score

### Requirement: Player is prompted to bank or roll again after each valid selection
The system SHALL present a "Bank (b) or Roll again (r)?" prompt after a valid selection and accept only "b" or "r" as input, re-prompting on anything else.

#### Scenario: Bank chosen
- **WHEN** player types "b"
- **THEN** the turn ends and control passes to the AI

#### Scenario: Roll chosen
- **WHEN** player types "r"
- **THEN** the remaining dice are rolled

### Requirement: Winner is announced at game end
The system SHALL print a clear one-line winner announcement when the game ends and then exit.

#### Scenario: Player wins
- **WHEN** the player's total reaches or exceeds 2000 after banking
- **THEN** output reads: "You win! Final scores — You: <n>, AI: <n>."

#### Scenario: AI wins
- **WHEN** the AI's total reaches or exceeds 2000 after banking
- **THEN** output reads: "AI wins. Final scores — You: <n>, AI: <n>."
