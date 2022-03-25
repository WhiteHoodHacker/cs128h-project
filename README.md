# cs128h-project

***Contributors:***

- Jacob Chang - `jechang3`
- Minh Duong - `minhd2`
- Dhiraj Kuttichirayil - `dhiraj2`
- Jane Liu - `janeliu2`

## Introduction

> Our group will be constructing a fully-featured chat app based in Rust :crab:

  We're trying to make an app with a fully featured backend that can allow communication between two people on the app, combined with a front end that makes the app easy to use.
  
  We chose this project because it takes advantage of several Rust libraries that allow Rust to applied into an interactive program by the end. We also thing think it would be cool to make (assuming that its functional by the end of the project :smile:)

## Systems Overview

> For the implementation of said chat app, we need a fully functional backend and frontend that can interact with eachother in order to create a functional app.

**Backend:**
  The backend for out chat app your realistically have all the nessecary features in order to have a working messaging feature. This includes:

- Ways to turn a message into a use data type in Rust (with all nessecary metadata)
- Method to transfer messages to another instance of the program (another user)
- Method to recieve an incoming message from another user
- Method of storing previous messages and eventually retrieving them

**Frontend:**
  The frontend end for our project should encapsulate the features of the chat-app into a user interface. This include:

- Visual representation of image exchange between users (typical format of most text services with outgoing messasing on right and incoming message on left)
- Button to send messages
- Alternate between different users and load older exchanged that have been stored

*Other*: We may also consider making a calendar feature into the app, that allows to map messages recieved over time in order to organize older data (if there's enough time left over)

## Technical Challenges

> Possible challenges we can forsee having include the following:

1. Transmitting data between two instances of the chat app properly
2. Designing a frontend for the app
   - Creating functional buttons, textboxes, and other interactive components
   - Designing the interface to be look visually appealing.

## References

This project was primarily inspired by this video from the *Let's Get Rusty* YouTube Channel:
  [Realtime Chat App in Rust!](https://www.youtube.com/watch?v=NS9Dh63i_Q4)