Install this package to do super fast value iteration with a transition probability array and a reward function.

devtools::install_github("cobriant/value_iteration@master")

valueiteration::value_iteration(
  n = 2, 
  reward_guess = c(1, 0), 
  transition_flat = c(1, 0, 0, 1, 0, 1, 1, 0), 
  rows = 2, 
  cols = 2, 
  depth = 2, 
  beta = 0.9, 
  out = "value" 
  )

Arguments:

n: length of the reward function

reward_guess: reward function

transition_flat: transition probability array (by row): make sure that the rows of each matrix sum to exactly 1

rows: number of rows of each matrix of the transition probability array

cols: number of columns of each matrix of the transition probability array

depth: number of matrices of the transition probability array

beta: discount factor (<1)

out: choose "value" or "value_action". value gives you the value function of length n; value_action gives you a matrix size n x depth.
