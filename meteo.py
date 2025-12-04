from datetime import datetime
import matplotlib.pyplot as plt
from meteostat import Point, Daily
import pandas as pd

# 1. Define the Time Period (Last 5 Years)
end_date = datetime.now()
start_date = end_date.replace(year=end_date.year - 5)

# 2. Define Locations (Latitude, Longitude)
# La Spezia coordinates
la_spezia_loc = Point(44.1025, 9.8241)
# Lucca coordinates
lucca_loc = Point(43.8429, 10.5027)

# 3. Fetch Data using Meteostat
print(f"Fetching data from {start_date.date()} to {end_date.date()}...")

# Get daily data
data_spezia = Daily(la_spezia_loc, start_date, end_date)
data_lucca = Daily(lucca_loc, start_date, end_date)

# Normalize the data (fetch and store in dataframe)
df_spezia = data_spezia.fetch()
df_lucca = data_lucca.fetch()

# 4. Data Processing
# We focus on the 'prcp' column (Precipitation in mm)
# Resample to 'Monthly' sum to make the chart readable
# 'ME' stands for Month End frequency
spezia_monthly = df_spezia['prcp'].resample('ME').sum()
lucca_monthly = df_lucca['prcp'].resample('ME').sum()

# 5. Plotting
plt.figure(figsize=(14, 7))

# Plot La Spezia
plt.plot(spezia_monthly.index, spezia_monthly, 
         label='La Spezia', color='#1f77b4', linewidth=2, marker='o', markersize=4)

# Plot Lucca
plt.plot(lucca_monthly.index, lucca_monthly, 
         label='Lucca', color='#ff7f0e', linewidth=2, marker='s', markersize=4, linestyle='--')

# Styling the plot
plt.title(f"Monthly Rainfall Comparison (Last 5 Years) La Spezia vs Lucca \nTOTAL: La Spezia: {df_spezia['prcp'].sum():.2f} mm, Lucca: {df_lucca['prcp'].sum():.2f} mm", fontsize=16)
plt.ylabel('Total Precipitation (mm)', fontsize=12)
plt.xlabel('Date', fontsize=12)
plt.legend(fontsize=12)
plt.grid(True, linestyle=':', alpha=0.6)

# Fill area under curves for better visibility
plt.fill_between(spezia_monthly.index, spezia_monthly, color='#1f77b4', alpha=0.1)
plt.fill_between(lucca_monthly.index, lucca_monthly, color='#ff7f0e', alpha=0.1)

plt.tight_layout()
plt.savefig('rainfall_comparison_la_spezia_lucca.png', dpi=300)
plt.show()

# Optional: Print total rainfall comparison
print(f"Total Rainfall (Last 5 Years) - La Spezia: {df_spezia['prcp'].sum():.2f} mm")
print(f"Total Rainfall (Last 5 Years) - Lucca:     {df_lucca['prcp'].sum():.2f} mm")