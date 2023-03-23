# HackerNews Alert

# A ChatGPT bot to respond to your GitHub Issues

[Deploy this function on flows.network](#deploy-the-hackernews-alert-app), and you will reveice HackerNews post alerts per hour according to your interests. 



## Deploy the HackerNews Alert App

To create this App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/hackernews-alert/) and go to flows.network to deploy your function. 

### Deploy the code on flow.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying this APP
3. Authenticate the [flows.network](https://flows.network/) to access the `hackernews-alert` repo you just forked. 
![image](https://user-images.githubusercontent.com/45785633/227176033-35a445d8-9e73-4d6d-a919-c68d64cc4075.png)

4. Click on the Advanced text and you will see more settings. Fill in the required Environment Variables. In this example, we have three variables. One is `KEYWORD`: fill in the topic you want to listen to,like `ChatGPT`. If you want to listen to multipule keyowrds, you can use `||` between each keyword. 
The other two variables: `WORKSPACE` and `CHANNEL`: fill in your own workspace and channel

![image](https://user-images.githubusercontent.com/45785633/227176580-b7e8d31d-b871-45b4-baee-312572615e8a.png)

5. At last, click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integration required by your flow.

![image](https://user-images.githubusercontent.com/45785633/227176699-a1ce1c05-02b9-411a-890f-ece033fde38e.png)

Here we can see, we need to configue one SaaS integration.

Click the "Connect/+ Add new authentication" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network` bot on your workspace. This workspace is the one you entered into the environment variables above.

> If you have authenticated the workspace before,you can see the purple Connect button turns gray Connected button. Just ingore this step and click Check button.

After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, the Hackernews alert App goes live. You will get a salck message at the 50th minute of every hour !

![image](https://user-images.githubusercontent.com/45785633/227177456-a51eacda-2f09-4206-874b-4dc73c3408d8.png)

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!


