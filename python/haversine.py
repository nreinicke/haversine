import haversine

# London
lat1 = 51.5007
lon1 = 0.1246

# New York 
lat2 = 40.6892
lon2 = 74.0445

distance = haversine.haversine(lat1, lon1, lat2, lon2)
print(f"Distance between London and New York is about {round(distance, 2)} kilometers")
