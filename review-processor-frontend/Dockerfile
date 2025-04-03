# Multi-stage Dockerfile for building and serving the Review Processor frontend
#
# This Dockerfile uses two stages:
# 1. Build stage: Compiles the Vue/Quasar application
# 2. Runner stage: Serves the built files using Nginx

# Build stage - compiles the application
FROM node:22-alpine AS build

# Set working directory for the build
WORKDIR /app

# Install Quasar CLI globally for building
RUN npm install -g @quasar/cli

# Copy application source code
COPY . .

# Install project dependencies
RUN npm install

# Build the application in production mode
RUN quasar build

# Runner stage - serves the application
FROM nginx:stable AS runner

# Copy the built application from build stage
COPY --from=build /app/dist/spa /usr/share/nginx/html

# Copy custom Nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose HTTP and HTTPS ports
EXPOSE 80 443

# Start Nginx in foreground mode
CMD ["nginx", "-g", "daemon off;"]