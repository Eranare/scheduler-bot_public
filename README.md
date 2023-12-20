## Scheduler-Bot
Simple scheduler bot for my game group.
/scheduler_setup adds the channel where the command is used along with the server to the bots database for it track player availability.
Users clicks on a react that lines up with the timeblock they are unavailable for. every 5 minutes the bot checks for changes in the database, and if availability has changed, removes the post holding the image and makes a new one accordingly.

![image](https://github.com/Eranare/scheduler-bot_public/assets/117918276/336c8782-6137-4004-917b-ac1ad58e0442)
![image](https://github.com/Eranare/scheduler-bot_public/assets/117918276/710d4c07-e2da-4fa9-b51e-2f87c079106b)

## Database
-Postgresql
