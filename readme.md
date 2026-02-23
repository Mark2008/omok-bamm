# Omok-Bamm

A bot project for the Omok (Gomoku) game.



### Status
Negamax and Alpha-Beta Pruning–based model implemented.



### Structure

- core  
  Game logic for Omok.

- bot  
  Predicts the next move.

  - model  
    Negamax and Alpha-Beta Pruning algorithm

  - eval  
    Evaluation function for the board

  - prune  
    Generates possible next board states

  - tt / hash  
    Zobrist hashing (currently not in use)