# Spec Delta

## Purpose

Defines the behavior of the CPU opponent: how it picks dice to keep from its roll and when it decides to bank versus roll again.

## ADDED Requirements

### Requirement: AI picks the highest-scoring valid subset of its roll
The system SHALL automatically select the dice combination from the AI's roll that yields the highest point value, using all selected dice in scoring combinations.

#### Scenario: AI roll contains multiple valid subsets
- **WHEN** the AI rolls and more than one valid scoring subset exists
- **THEN** the AI selects the subset with the highest score

#### Scenario: AI roll is a Farkle
- **WHEN** the AI rolls and no valid scoring subset exists
- **THEN** Farkle is declared, the AI's turn score is forfeited, and the turn passes to the player

### Requirement: AI banks when turn score meets threshold or dice count is low
The system SHALL end the AI's turn and bank its accumulated turn score when that score reaches or exceeds 350 points, or when two or fewer dice remain to roll.

#### Scenario: AI exceeds bank threshold
- **WHEN** the AI's accumulated turn score reaches or exceeds 350
- **THEN** the AI banks and its turn ends

#### Scenario: AI has two or fewer dice remaining
- **WHEN** the AI has selected dice and two or fewer dice remain
- **THEN** the AI banks and its turn ends

### Requirement: AI narrates its moves tersely
The system SHALL print one line per AI action (selection, bank, Farkle) in a brief, factual format.

#### Scenario: AI keeps dice and rolls again
- **WHEN** the AI selects dice and chooses to roll again
- **THEN** the output reads: "AI keeps [<values>] (+<points>), rolls <n> dice."

#### Scenario: AI banks
- **WHEN** the AI decides to bank
- **THEN** the output reads: "AI banks. Score: <total>."
