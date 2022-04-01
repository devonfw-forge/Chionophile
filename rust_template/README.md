# Rust API Template

## Introduction 

This folder contains an API Template that will serve as a base for all Rust based projects across Capgemini. It uses the Actix framework and is 
by default configured to use a PostgreSQL database with the Diesel ORM/Query Builder.

## Structure

The config file inside the .cargo folder stores the environment variables.

The project is divided in two main packages, based in the DevonFW Java project structure:
* API
* Core

API contains all the standarized and entity related traits, serving as interfaces for the implementations present in the Core package.

The Core, along with the previously mentioned implementations, also includes configuration and state management files.

## Current functionality

The template as given only contains CRUD functionality for a basic User entity. It is meant to be built upon and as such, more robust security features and
complex authentication and logic are meant to be implemented by the development team.
