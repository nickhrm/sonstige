# Use an official Python runtime as a parent image
FROM python:3.9-slim

# Set the working directory
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Install any needed packages specified in requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Make the calculate.py and getNumbers.py executable
RUN chmod +x calculate.py getNumbers.py

# Run the command on container startup
CMD ["cron", "-f"]