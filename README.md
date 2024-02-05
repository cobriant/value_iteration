Install this package to do super fast value iteration with a transition probability array and a reward function.

devtools::install_github("cobriant/value_iteration@master")

valueiteration::value_iteration(
  n = 2, # length of the reward function
  reward_guess = c(1, 0), # reward function
  transition_flat = c(1, 0, 0, 1, 0, 1, 1, 0), # transition probability array (by row): make sure that the rows of each matrix sum to exactly 1
  rows = 2, # number of rows of each matrix of the transition probability array
  cols = 2, # number of columns of each matrix of the transition probability array
  depth = 2, # number of matrices of the transition probability array
  beta = 0.9, # discount factor (<1)
  out = "value" # choose "value" or "value_action". value gives you the value function of length n; value_action gives you a matrix size n x depth.
  )
