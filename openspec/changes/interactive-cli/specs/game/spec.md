# Spec Delta

## Purpose

Manages the full state of a 1v1 Farkle game: rolling dice, tracking scores per turn and in total, enforcing Farkle and hot-dice rules, and declaring the winner when a player reaches 2000 points.

## ADDED Requirements

### Requirement: Game initializes with two players and six dice
The system SHALL start each game with both player and AI scores at zero, six dice available, and the player taking the first turn.

#### Scenario: Fresh game state
- **WHEN** a new game is started
- **THEN** both player score and AI score are 0, dice count is 6, and accumulated turn score is 0

### Requirement: Player selects scoring dice by position
The system SHALL accept a selection of dice positions (1–6) and validate that every selected die participates in a scoring combination before accepting the selection.

#### Scenario: Valid scoring selection accepted
- **WHEN** the player selects positions whose face values form a valid scoring combination
- **THEN** the selection is accepted, the turn score increases by the selection's point value, and selected dice are removed from the available pool

#### Scenario: Invalid selection rejected
- **WHEN** the player selects positions that include a non-scoring die
- **THEN** the selection is rejected with an error message and the player is re-prompted without losing their accumulated turn score

### Requirement: Farkle ends a turn with no score
The system SHALL detect Farkle — when no valid scoring subset exists in the newly rolled dice — and immediately end the player's turn, discarding the accumulated turn score.

#### Scenario: No scoring subset in roll
- **WHEN** the dice are rolled and no subset of them forms a valid scoring combination
- **THEN** "Farkle!" is displayed, the accumulated turn score is forfeited, and the turn passes to the next player

### Requirement: Hot dice reset the pool to six
The system SHALL reset the available dice count to six when the player has selected all remaining dice in a valid scoring combination, allowing them to roll again with a full set.

#### Scenario: All dice selected in valid combination
- **WHEN** the player selects dice and no unselected dice remain
- **THEN** the dice pool resets to six and the player may continue their turn

### Requirement: Player may bank to claim turn score
The system SHALL allow the player to bank at any point after a valid selection, adding the accumulated turn score to their total and ending their turn.

#### Scenario: Bank after scoring
- **WHEN** the player chooses to bank
- **THEN** the accumulated turn score is added to the player's total score and the turn passes to the next player

### Requirement: First player to reach 2000 wins
The system SHALL end the game and declare the winner when a player's total score reaches or exceeds 2000 after banking.

#### Scenario: Player banks to reach 2000
- **WHEN** a player banks and their total score reaches or exceeds 2000
- **THEN** the game ends and that player is declared the winner
