import random
import gymnasium
import time

def timer_decorator(func):
    def wrapper(*args, **kwargs):
        start_time = time.time()  # Record the start time
        result = func(*args, **kwargs)  # Call the original function
        end_time = time.time()  # Record the end time
        execution_time = end_time - start_time  # Calculate execution time
        print(f"Function '{func.__name__}' executed in {execution_time:.4f} seconds")
        return result  # Return the result of the original function
    return wrapper

def take_actions(env, actions) -> None:
    for action in actions:
        env.step(action)

@timer_decorator
def take_n_steps_vanilla(n: int) -> None:
    env = gymnasium.make('Taxi-v3')
    env.reset()
    for _ in range(n):
        random_action = random.randint(0, 3)
        next_state, reward, terminated, truncated, info = env.step(random_action)

def main():
    n_steps = 10000000
    take_n_steps_vanilla(n_steps)

if __name__ == '__main__':
    main()