import pandas as pd
import matplotlib.pyplot as plt

data = pd.read_csv("../data.csv", names=["timestamp", "ph", "conductivity", "temperature"])

plt.figure(figsize=(10,6))
plt.plot(data.timestamp, data.ph, label="pH")
plt.axhline(6.5, color="red", linestyle="--", alpha=0.5)
plt.axhline(8.5, color="red", linestyle="--", alpha=0.5)
plt.title("pH Readings Over Time")
plt.xlabel("Time")
plt.ylabel("pH")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("ph_plot.png")
print("Saved ph_plot.png")
