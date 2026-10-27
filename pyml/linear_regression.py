import pandas as pd
import numpy as np
import matplotlib.pyplot as py

# Get and read data from file with pandas
try:
    df = pd.read_csv("data.csv")
    hours = df["Hours"].to_numpy().astype(np.float64)
    pay = df["Pay"].to_numpy().astype(np.float64)

    X = hours
    y = pay

    learning_rate = 0.0001 
    w = 0.0
    b = 0.0

  
    # Gradient Descent Optimization Loop
    for epoch in range(8000):
        y_prediction = w * X + b
        error = y_prediction - y

        dw = 2 * np.mean(error * X)
        db = 2 * np.mean(error)

        w -= learning_rate * dw
        b -= learning_rate * db

    print(f"final weight : {w}")
    print(f"final bias: {b}")

    

except FileNotFoundError:
    print("Error: 'data.csv' file not found. Check the path and file name.")
except KeyError as e:
    print(f"Error: Column header {e} not found in CSV. Double-check your column names!")
except Exception as e:
    print(f"An unexpected error occurred: {e}")
