# Use an official Rust image as a parent image
FROM rust:latest

# Install Shell In A Box
RUN apt-get update && apt-get install -y shellinabox

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled executables to the working directory
COPY ./1_lecture /usr/src/app
COPY ./2_quiz /usr/src/app

# Expose the port used by Shell In A Box
EXPOSE 4200

# Run Shell In A Box on container startup
CMD ["shellinaboxd", "-t", "-p", "4200", "--no-beep", "-s", "/:LOGIN"]

