<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Automation Step by Step                _b66b9e</name>
   <tag></tag>
   <elementGuidId>62b935af-e58c-4d2e-83d8-84a1efabd335</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.wrapper.clearfix</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Show Me'])[1]/following::div[1]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>0cbe6ad7-54b3-4920-be13-7976258e3f35</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>wrapper clearfix</value>
      <webElementGuid>6a348fc6-b91b-48ca-9ff8-d5ca00736033</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>aria-hidden</name>
      <type>Main</type>
      <value>false</value>
      <webElementGuid>892ba21f-3c49-45b7-be20-71be213da908</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

	
		
	

        
        
                        

                

                                            
                    Automation Step by Step                        
                    
                    
                                            AUTOMATION | TESTING | DEVOPS | CI
                    
                
                            

                

		        

			Menu

			
				Home
JMeter

	JMeter Beginner
	JMeter Intermediate
	JMeter Advanced


Selenium

	Selenium Introduction
	Selenium TIPS
	Selenium with Python
	Selenium Builder


API

	SOAP Web Services (API)
	REST Web Services (API)


DevOps

	Jenkins
	Docker
	Git and GitHub


Tools

	Redis
	Katalon Studio
	JIRA
	Maven
	Tomcat
	Java for Automation


Training
Udemy
About
			

		
		
	
	

	

	
				
	
	

	
		A Story of OAuth
		April 30, 2020Mickey and Minnie StoriesComments: 0	

		


One day Mickey was cleaning his car when he heard Minnie calling him.
Minnie – Hey Mickey!
Mickey – Oh Minnie, Hi!
Minnie – I need to talk to you.
Mickey – Yeah, tell me.
Minnie – Do you know about this OAuth.
Mickey – Yes, why do you ask?
Minnie – I need to explain this to a friend who is preparing for an interview. And need to understand from scratch and basics. Can you help?
Mickey – Certainly I can. Do you like to go for a drive?
Minnie – Sure let’s go.

Minnie – Okay Mickey! tell me now
Mickey – So here is the Wikipedia definition of OAuth
“OAuth is an open standard for access delegation, commonly used as a way for Internet users to grant websites or applications access to their information on other websites but without giving them the passwords”
Minnie – I do not get a word here. Moreover, I could have read that on the internet, Why do I need you then, Mickey!
Mickey – Okay Sorry! so Minnie when was the last time you went for a Holiday.
Minnie – It was a few months ago, but can I tell you the details later? Can we focus on OAuth? I don’t have all day to roam around in your car.
Mickey – Oh-kay! Cool, I am coming to that. I just wanted to explain with an example.
Minnie – Oh! Sorry! I went to Corsica a few months ago. Please Continue…

Mickey – Great, So tell me how did you book and access your room in the hotel.
Minnie – Well I had done online booking. And when I reached the hotel, the receptionist validated my identity and gave me the room key.
Mickey – Exactly, now with that Key, you can access your room
Minnie – That’s right.
Mickey – Now think of your room as a resource and this resource is owned by the hotel. Am I right?
Minnie – Absolutely
Mickey – So we have a resource (room) a resource owner (hotel) and then a person who needs to access or use the resource that is you (the Client)
Minnie – That’s all understandable.
Mickey – Great! Now to get access to the resource, you need to get a token (i.e. the key) from the resource owner (hotel). To be specific the receptionist of the hotel.
Minnie – Yes
Mickey – And when you access the room (resource) using the key (token), It gets validated from the resource owner (hotel) and if this all is fine, your room gets opened and you have access to the room (resource)

Minnie – Yes, all this I can understand.
Mickey – Also the key can get you access to a specific resource (room) and not other resources that are owned by the resource owner (hotel)
Minnie – Yes! Of course
Mickey – Okay, so there are 4 terminologies we discussed, Can you tell all?

Minnie – Yes! here you go:
Resource = Room
Resource Owner = Hotel
Client = Minnie (Me)
Token = Key
Mickey – Now just think this flow happening between services on the web.
Minnie – What! between services. How
Mickey – Well you must have seen this in your daily life.
Minnie – Where
Mickey – Have you seen when you try to log in on some applications or website. They have the option to either SignUp or you can log in using your Google or Facebook account. You see options like “Sign in with Google” or “Sign in with Facebook”

Minnie – Yes I see this a lot of times
Mickey – Let’s understand with an example.
 if you want to share your photos from your google drive to another application (let’s say a photo printing application), This application asks for access to your google photos and then you can allow or deny it.
Minnie – Yes, I know.
Mickey – Okay so let’s talk about the Photo printing application. You need to get your photos printed. You goto the Photo printing app and ask it to get your photos from your google photos present on your google drive. 
So here, there are the 4 steps involved:
Step 1: The Photo Printing application will ask Google drive to get access to your photos

Step 2: The Google Drive service will ask you whether to allow Photo Printing application to access your photos. 
(It will show you the permissions this application will have once you allow)
(assuming you say Yes)

Step 3: The Google Drive Service now hands over a key to the Photo Printing Service called authorization token
“This is called providing secured delegated access”
(Remember using the key you could just access the hotel room)

Step 4: Now the Photo Printing Service  can get access to your photos using the access key or token


Minnie – Yes, this all looks understandable now.
Mickey – So the process of this Authorization is done based on an open standard protocol called OAuth
Mickey – Tell me more.
Mickey – okay now let us try to assign all our 4 roles of Resource, Resource Owner, Client and Token in this scenario
Minnie – This is getting interesting
Mickey – So we have 4 actors here
Photo Printing application
Your Photos (on Google Drive)
Google Drive
Authorization Token
Now tell me who is what
Minnie – Okay, so
Photo Printing application is the Client
Your Photos (on Google Drive) is the Resource
Google Drive is the Resource owner
Authorization Token is the key or Token
Mickey – Well done. Full marks.
Minnie – Thank You! (Blushing)

Mickey – So here are the key points

OAuth is all about Authorization and not Authentication
(hope you remember the Story of Authentication vs Authorization)
OAuth is for authorization between services
OAuth is an open standard for access delegation

(Remember it allows limited access as per the permission)
It is an open standard so it can be used by any services and so all of them can use a standard way to communicate with each other.
Now there are 2 versions OAuth 1.0 and OAuth 2.0.
OAuth 2.0 is the latest implementation and is widely and commonly used.
Minnie – Now I am getting this well.
Mickey – Now check this Wikipedia definition again
“OAuth is an open standard for access delegation, commonly used as a way for Internet users to grant websites or applications access to their information on other websites but without giving them the passwords”
Minnie – Yes, now this is making meaning
Mickey – Another way of saying this:
“OAuth is an open-standard authorization protocol or framework that describes how unrelated servers and services can safely allow authenticated access to their assets without actually sharing the login credential“
Mickey – I got it.
Mickey – Well, so shall we go and explain this to your friend?
Minnie – I am in no hurry now. Let’s go to the riverside and spend some time together.
Mickey – As you say, Ma’am. Let’s Go.
Minnie – Thank You! I feel so good now.
Mickey – I am always here for you 🙂

about the author  |  more stories


214
			
			
			
			
			
			Share this:TwitterFacebookGoogleSkypeLinkedIn

	Related
A Story of Automation FrameworkJuly 20, 2020In &quot;Mickey and Minnie Stories&quot;A Story of Artificial Intelligence (AI)April 26, 2020In &quot;Mickey and Minnie Stories&quot;A Story of JSONMay 4, 2020In &quot;Mickey and Minnie Stories&quot;
	 		Tags: explain oauth with example, how oauth works, oauth analogy, oauth for beginners, what is oauth
	
		

			

			
				
				
				
				
			

			
			
			

			
				
				
				
				
					
					
				
			

		
		
		
	

		


	 
	


	

    
    
    

Latest News   Stories by Raghav  -  click here                    |             Click here for Udemy Courses by Raghav           |           To Invite Raghav for Events or Corporate Training email - training.raghav@gmail.com      |            Stories by Raghav  -  click here                    |             Click here for Udemy Courses by Raghav           |           To Invite Raghav for Events or Corporate Training email - training.raghav@gmail.com      |         jQuery(function(){jQuery('.marquee-hsas-widget-49').marquee({allowCss3Support: true,css3easing: 'linear',easing: 'linear',delayBeforeStart: 2000,direction: 'left',duplicated: true,duration: 6000,gap: 100,pauseOnCycle: true,pauseOnHover: true,startVisible: true});});Learn something new step by stepJoin my YouTube family

Archives
Mickey &amp; Minnie Stories
Online Courses
Training by Raghav
Quiz Time
Udemy Courses
Selenium
Jenkins
JMeter
Docker
Git and GitHub
Java for Automation
Katalon Studio
Maven
Redis
REST Web Services (API)
SOAP Web Services (API)
Selenium Introduction
UI Testing
API Testing
Mobile Testing
CI CD DevOps
Performance Testing
Newsletter
YouTube Playlists
SoapUI MCQ Quiz
Katalon Studio MCQ Quiz
Free Online Tutorials
Groovy Quiz
Jenkins Foundation Training
Groovy Beginners Training
SoapUI Beginners Training
Postman Beginners Training
About
Newsletter
The Testing QUIZ
BDD QUIZ
Python Quiz
Cypress Quiz
Corporate Training
IntelliJ IDEA QUIZ
REST Assured QUIZ
Introduction to Mobile Testing &amp; Appium
Selenium Quiz
Demo Quiz 1
Gatling Quiz 1
Gatling Quiz
Gatling Quiz 2
Hindi Tutorials
Select Language​▼visitor count2,953,924 CategoriesCategories
	Select Category
	Author : Raghav Pal  (3)
	Automation  (166)
	CI  (22)
	DevOps  (23)
	Docker  (6)
	Interview Questions  (4)
	Java  (11)
	Jenkins  (16)
	JMeter  (38)
	JMeter Issues  (2)
	Mickey and Minnie Stories  (49)
	Performance Testing  (36)
	Python  (1)
	Selenium  (28)
	Stories  (2)
	Testing  (41)
	Tools  (5)



/* &lt;![CDATA[ */
(function() {
	var dropdown = document.getElementById( &quot;cat&quot; );
	function onCatChange() {
		if ( dropdown.options[ dropdown.selectedIndex ].value > 0 ) {
			dropdown.parentNode.submit();
		}
	}
	dropdown.onchange = onCatChange;
})();
/* ]]> */


			
	









	
		
		Archives
Mickey &amp; Minnie Stories
Online Courses
Training by Raghav
Quiz Time
Udemy Courses
Selenium
Jenkins
JMeter
Docker
Git and GitHub
Java for Automation
Katalon Studio
Maven
Redis
REST Web Services (API)
SOAP Web Services (API)
Selenium Introduction
UI Testing
API Testing
Mobile Testing
CI CD DevOps
Performance Testing
Newsletter
YouTube Playlists
SoapUI MCQ Quiz
Katalon Studio MCQ Quiz
Free Online Tutorials
Groovy Quiz
Jenkins Foundation Training
Groovy Beginners Training
SoapUI Beginners Training
Postman Beginners Training
About
Newsletter
The Testing QUIZ
BDD QUIZ
Python Quiz
Cypress Quiz
Corporate Training
IntelliJ IDEA QUIZ
REST Assured QUIZ
Introduction to Mobile Testing &amp; Appium
Selenium Quiz
Demo Quiz 1
Gatling Quiz 1
Gatling Quiz
Gatling Quiz 2
Hindi Tutorials
	
	
	    	
		
			
				Automation Step by Step © 2023				
				
							
		

		
			
				Theme by				WP Puzzle
			
						
		
	
    	



</value>
      <webElementGuid>81a495dd-45cc-4160-8e92-1300c906d6e4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;post-template-default single single-post postid-4456 single-format-standard layout-&quot;]/div[@class=&quot;wrapper clearfix&quot;]</value>
      <webElementGuid>dbea89fa-01b3-478a-bdd1-66a6eb0df8f2</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Show Me'])[1]/following::div[1]</value>
      <webElementGuid>c0a71c41-7ebf-4378-bb5a-05a9497b3c55</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Read this Story'])[1]/following::div[2]</value>
      <webElementGuid>c78d829b-813f-4d48-96c1-c893c3a215c0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body/div[2]</value>
      <webElementGuid>bee959f5-7cdf-4ee2-bec1-9711e6648491</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;

	
		
	

        
        
                        

                

                                            
                    Automation Step by Step                        
                    
                    
                                            AUTOMATION | TESTING | DEVOPS | CI
                    
                
                            

                

		        

			Menu

			
				Home
JMeter

	JMeter Beginner
	JMeter Intermediate
	JMeter Advanced


Selenium

	Selenium Introduction
	Selenium TIPS
	Selenium with Python
	Selenium Builder


API

	SOAP Web Services (API)
	REST Web Services (API)


DevOps

	Jenkins
	Docker
	Git and GitHub


Tools

	Redis
	Katalon Studio
	JIRA
	Maven
	Tomcat
	Java for Automation


Training
Udemy
About
			

		
		
	
	

	

	
				
	
	

	
		A Story of OAuth
		April 30, 2020Mickey and Minnie StoriesComments: 0	

		


One day Mickey was cleaning his car when he heard Minnie calling him.
Minnie – Hey Mickey!
Mickey – Oh Minnie, Hi!
Minnie – I need to talk to you.
Mickey – Yeah, tell me.
Minnie – Do you know about this OAuth.
Mickey – Yes, why do you ask?
Minnie – I need to explain this to a friend who is preparing for an interview. And need to understand from scratch and basics. Can you help?
Mickey – Certainly I can. Do you like to go for a drive?
Minnie – Sure let’s go.

Minnie – Okay Mickey! tell me now
Mickey – So here is the Wikipedia definition of OAuth
“OAuth is an open standard for access delegation, commonly used as a way for Internet users to grant websites or applications access to their information on other websites but without giving them the passwords”
Minnie – I do not get a word here. Moreover, I could have read that on the internet, Why do I need you then, Mickey!
Mickey – Okay Sorry! so Minnie when was the last time you went for a Holiday.
Minnie – It was a few months ago, but can I tell you the details later? Can we focus on OAuth? I don’t have all day to roam around in your car.
Mickey – Oh-kay! Cool, I am coming to that. I just wanted to explain with an example.
Minnie – Oh! Sorry! I went to Corsica a few months ago. Please Continue…

Mickey – Great, So tell me how did you book and access your room in the hotel.
Minnie – Well I had done online booking. And when I reached the hotel, the receptionist validated my identity and gave me the room key.
Mickey – Exactly, now with that Key, you can access your room
Minnie – That’s right.
Mickey – Now think of your room as a resource and this resource is owned by the hotel. Am I right?
Minnie – Absolutely
Mickey – So we have a resource (room) a resource owner (hotel) and then a person who needs to access or use the resource that is you (the Client)
Minnie – That’s all understandable.
Mickey – Great! Now to get access to the resource, you need to get a token (i.e. the key) from the resource owner (hotel). To be specific the receptionist of the hotel.
Minnie – Yes
Mickey – And when you access the room (resource) using the key (token), It gets validated from the resource owner (hotel) and if this all is fine, your room gets opened and you have access to the room (resource)

Minnie – Yes, all this I can understand.
Mickey – Also the key can get you access to a specific resource (room) and not other resources that are owned by the resource owner (hotel)
Minnie – Yes! Of course
Mickey – Okay, so there are 4 terminologies we discussed, Can you tell all?

Minnie – Yes! here you go:
Resource = Room
Resource Owner = Hotel
Client = Minnie (Me)
Token = Key
Mickey – Now just think this flow happening between services on the web.
Minnie – What! between services. How
Mickey – Well you must have seen this in your daily life.
Minnie – Where
Mickey – Have you seen when you try to log in on some applications or website. They have the option to either SignUp or you can log in using your Google or Facebook account. You see options like “Sign in with Google” or “Sign in with Facebook”

Minnie – Yes I see this a lot of times
Mickey – Let’s understand with an example.
 if you want to share your photos from your google drive to another application (let’s say a photo printing application), This application asks for access to your google photos and then you can allow or deny it.
Minnie – Yes, I know.
Mickey – Okay so let’s talk about the Photo printing application. You need to get your photos printed. You goto the Photo printing app and ask it to get your photos from your google photos present on your google drive. 
So here, there are the 4 steps involved:
Step 1: The Photo Printing application will ask Google drive to get access to your photos

Step 2: The Google Drive service will ask you whether to allow Photo Printing application to access your photos. 
(It will show you the permissions this application will have once you allow)
(assuming you say Yes)

Step 3: The Google Drive Service now hands over a key to the Photo Printing Service called authorization token
“This is called providing secured delegated access”
(Remember using the key you could just access the hotel room)

Step 4: Now the Photo Printing Service  can get access to your photos using the access key or token


Minnie – Yes, this all looks understandable now.
Mickey – So the process of this Authorization is done based on an open standard protocol called OAuth
Mickey – Tell me more.
Mickey – okay now let us try to assign all our 4 roles of Resource, Resource Owner, Client and Token in this scenario
Minnie – This is getting interesting
Mickey – So we have 4 actors here
Photo Printing application
Your Photos (on Google Drive)
Google Drive
Authorization Token
Now tell me who is what
Minnie – Okay, so
Photo Printing application is the Client
Your Photos (on Google Drive) is the Resource
Google Drive is the Resource owner
Authorization Token is the key or Token
Mickey – Well done. Full marks.
Minnie – Thank You! (Blushing)

Mickey – So here are the key points

OAuth is all about Authorization and not Authentication
(hope you remember the Story of Authentication vs Authorization)
OAuth is for authorization between services
OAuth is an open standard for access delegation

(Remember it allows limited access as per the permission)
It is an open standard so it can be used by any services and so all of them can use a standard way to communicate with each other.
Now there are 2 versions OAuth 1.0 and OAuth 2.0.
OAuth 2.0 is the latest implementation and is widely and commonly used.
Minnie – Now I am getting this well.
Mickey – Now check this Wikipedia definition again
“OAuth is an open standard for access delegation, commonly used as a way for Internet users to grant websites or applications access to their information on other websites but without giving them the passwords”
Minnie – Yes, now this is making meaning
Mickey – Another way of saying this:
“OAuth is an open-standard authorization protocol or framework that describes how unrelated servers and services can safely allow authenticated access to their assets without actually sharing the login credential“
Mickey – I got it.
Mickey – Well, so shall we go and explain this to your friend?
Minnie – I am in no hurry now. Let’s go to the riverside and spend some time together.
Mickey – As you say, Ma’am. Let’s Go.
Minnie – Thank You! I feel so good now.
Mickey – I am always here for you 🙂

about the author  |  more stories


214
			
			
			
			
			
			Share this:TwitterFacebookGoogleSkypeLinkedIn

	Related
A Story of Automation FrameworkJuly 20, 2020In &quot;Mickey and Minnie Stories&quot;A Story of Artificial Intelligence (AI)April 26, 2020In &quot;Mickey and Minnie Stories&quot;A Story of JSONMay 4, 2020In &quot;Mickey and Minnie Stories&quot;
	 		Tags: explain oauth with example, how oauth works, oauth analogy, oauth for beginners, what is oauth
	
		

			

			
				
				
				
				
			

			
			
			

			
				
				
				
				
					
					
				
			

		
		
		
	

		


	 
	


	

    
    
    

Latest News   Stories by Raghav  -  click here                    |             Click here for Udemy Courses by Raghav           |           To Invite Raghav for Events or Corporate Training email - training.raghav@gmail.com      |            Stories by Raghav  -  click here                    |             Click here for Udemy Courses by Raghav           |           To Invite Raghav for Events or Corporate Training email - training.raghav@gmail.com      |         jQuery(function(){jQuery(&quot; , &quot;'&quot; , &quot;.marquee-hsas-widget-49&quot; , &quot;'&quot; , &quot;).marquee({allowCss3Support: true,css3easing: &quot; , &quot;'&quot; , &quot;linear&quot; , &quot;'&quot; , &quot;,easing: &quot; , &quot;'&quot; , &quot;linear&quot; , &quot;'&quot; , &quot;,delayBeforeStart: 2000,direction: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,duplicated: true,duration: 6000,gap: 100,pauseOnCycle: true,pauseOnHover: true,startVisible: true});});Learn something new step by stepJoin my YouTube family

Archives
Mickey &amp; Minnie Stories
Online Courses
Training by Raghav
Quiz Time
Udemy Courses
Selenium
Jenkins
JMeter
Docker
Git and GitHub
Java for Automation
Katalon Studio
Maven
Redis
REST Web Services (API)
SOAP Web Services (API)
Selenium Introduction
UI Testing
API Testing
Mobile Testing
CI CD DevOps
Performance Testing
Newsletter
YouTube Playlists
SoapUI MCQ Quiz
Katalon Studio MCQ Quiz
Free Online Tutorials
Groovy Quiz
Jenkins Foundation Training
Groovy Beginners Training
SoapUI Beginners Training
Postman Beginners Training
About
Newsletter
The Testing QUIZ
BDD QUIZ
Python Quiz
Cypress Quiz
Corporate Training
IntelliJ IDEA QUIZ
REST Assured QUIZ
Introduction to Mobile Testing &amp; Appium
Selenium Quiz
Demo Quiz 1
Gatling Quiz 1
Gatling Quiz
Gatling Quiz 2
Hindi Tutorials
Select Language​▼visitor count2,953,924 CategoriesCategories
	Select Category
	Author : Raghav Pal  (3)
	Automation  (166)
	CI  (22)
	DevOps  (23)
	Docker  (6)
	Interview Questions  (4)
	Java  (11)
	Jenkins  (16)
	JMeter  (38)
	JMeter Issues  (2)
	Mickey and Minnie Stories  (49)
	Performance Testing  (36)
	Python  (1)
	Selenium  (28)
	Stories  (2)
	Testing  (41)
	Tools  (5)



/* &lt;![CDATA[ */
(function() {
	var dropdown = document.getElementById( &quot;cat&quot; );
	function onCatChange() {
		if ( dropdown.options[ dropdown.selectedIndex ].value > 0 ) {
			dropdown.parentNode.submit();
		}
	}
	dropdown.onchange = onCatChange;
})();
/* ]]> */


			
	









	
		
		Archives
Mickey &amp; Minnie Stories
Online Courses
Training by Raghav
Quiz Time
Udemy Courses
Selenium
Jenkins
JMeter
Docker
Git and GitHub
Java for Automation
Katalon Studio
Maven
Redis
REST Web Services (API)
SOAP Web Services (API)
Selenium Introduction
UI Testing
API Testing
Mobile Testing
CI CD DevOps
Performance Testing
Newsletter
YouTube Playlists
SoapUI MCQ Quiz
Katalon Studio MCQ Quiz
Free Online Tutorials
Groovy Quiz
Jenkins Foundation Training
Groovy Beginners Training
SoapUI Beginners Training
Postman Beginners Training
About
Newsletter
The Testing QUIZ
BDD QUIZ
Python Quiz
Cypress Quiz
Corporate Training
IntelliJ IDEA QUIZ
REST Assured QUIZ
Introduction to Mobile Testing &amp; Appium
Selenium Quiz
Demo Quiz 1
Gatling Quiz 1
Gatling Quiz
Gatling Quiz 2
Hindi Tutorials
	
	
	    	
		
			
				Automation Step by Step © 2023				
				
							
		

		
			
				Theme by				WP Puzzle
			
						
		
	
    	



&quot;) or . = concat(&quot;

	
		
	

        
        
                        

                

                                            
                    Automation Step by Step                        
                    
                    
                                            AUTOMATION | TESTING | DEVOPS | CI
                    
                
                            

                

		        

			Menu

			
				Home
JMeter

	JMeter Beginner
	JMeter Intermediate
	JMeter Advanced


Selenium

	Selenium Introduction
	Selenium TIPS
	Selenium with Python
	Selenium Builder


API

	SOAP Web Services (API)
	REST Web Services (API)


DevOps

	Jenkins
	Docker
	Git and GitHub


Tools

	Redis
	Katalon Studio
	JIRA
	Maven
	Tomcat
	Java for Automation


Training
Udemy
About
			

		
		
	
	

	

	
				
	
	

	
		A Story of OAuth
		April 30, 2020Mickey and Minnie StoriesComments: 0	

		


One day Mickey was cleaning his car when he heard Minnie calling him.
Minnie – Hey Mickey!
Mickey – Oh Minnie, Hi!
Minnie – I need to talk to you.
Mickey – Yeah, tell me.
Minnie – Do you know about this OAuth.
Mickey – Yes, why do you ask?
Minnie – I need to explain this to a friend who is preparing for an interview. And need to understand from scratch and basics. Can you help?
Mickey – Certainly I can. Do you like to go for a drive?
Minnie – Sure let’s go.

Minnie – Okay Mickey! tell me now
Mickey – So here is the Wikipedia definition of OAuth
“OAuth is an open standard for access delegation, commonly used as a way for Internet users to grant websites or applications access to their information on other websites but without giving them the passwords”
Minnie – I do not get a word here. Moreover, I could have read that on the internet, Why do I need you then, Mickey!
Mickey – Okay Sorry! so Minnie when was the last time you went for a Holiday.
Minnie – It was a few months ago, but can I tell you the details later? Can we focus on OAuth? I don’t have all day to roam around in your car.
Mickey – Oh-kay! Cool, I am coming to that. I just wanted to explain with an example.
Minnie – Oh! Sorry! I went to Corsica a few months ago. Please Continue…

Mickey – Great, So tell me how did you book and access your room in the hotel.
Minnie – Well I had done online booking. And when I reached the hotel, the receptionist validated my identity and gave me the room key.
Mickey – Exactly, now with that Key, you can access your room
Minnie – That’s right.
Mickey – Now think of your room as a resource and this resource is owned by the hotel. Am I right?
Minnie – Absolutely
Mickey – So we have a resource (room) a resource owner (hotel) and then a person who needs to access or use the resource that is you (the Client)
Minnie – That’s all understandable.
Mickey – Great! Now to get access to the resource, you need to get a token (i.e. the key) from the resource owner (hotel). To be specific the receptionist of the hotel.
Minnie – Yes
Mickey – And when you access the room (resource) using the key (token), It gets validated from the resource owner (hotel) and if this all is fine, your room gets opened and you have access to the room (resource)

Minnie – Yes, all this I can understand.
Mickey – Also the key can get you access to a specific resource (room) and not other resources that are owned by the resource owner (hotel)
Minnie – Yes! Of course
Mickey – Okay, so there are 4 terminologies we discussed, Can you tell all?

Minnie – Yes! here you go:
Resource = Room
Resource Owner = Hotel
Client = Minnie (Me)
Token = Key
Mickey – Now just think this flow happening between services on the web.
Minnie – What! between services. How
Mickey – Well you must have seen this in your daily life.
Minnie – Where
Mickey – Have you seen when you try to log in on some applications or website. They have the option to either SignUp or you can log in using your Google or Facebook account. You see options like “Sign in with Google” or “Sign in with Facebook”

Minnie – Yes I see this a lot of times
Mickey – Let’s understand with an example.
 if you want to share your photos from your google drive to another application (let’s say a photo printing application), This application asks for access to your google photos and then you can allow or deny it.
Minnie – Yes, I know.
Mickey – Okay so let’s talk about the Photo printing application. You need to get your photos printed. You goto the Photo printing app and ask it to get your photos from your google photos present on your google drive. 
So here, there are the 4 steps involved:
Step 1: The Photo Printing application will ask Google drive to get access to your photos

Step 2: The Google Drive service will ask you whether to allow Photo Printing application to access your photos. 
(It will show you the permissions this application will have once you allow)
(assuming you say Yes)

Step 3: The Google Drive Service now hands over a key to the Photo Printing Service called authorization token
“This is called providing secured delegated access”
(Remember using the key you could just access the hotel room)

Step 4: Now the Photo Printing Service  can get access to your photos using the access key or token


Minnie – Yes, this all looks understandable now.
Mickey – So the process of this Authorization is done based on an open standard protocol called OAuth
Mickey – Tell me more.
Mickey – okay now let us try to assign all our 4 roles of Resource, Resource Owner, Client and Token in this scenario
Minnie – This is getting interesting
Mickey – So we have 4 actors here
Photo Printing application
Your Photos (on Google Drive)
Google Drive
Authorization Token
Now tell me who is what
Minnie – Okay, so
Photo Printing application is the Client
Your Photos (on Google Drive) is the Resource
Google Drive is the Resource owner
Authorization Token is the key or Token
Mickey – Well done. Full marks.
Minnie – Thank You! (Blushing)

Mickey – So here are the key points

OAuth is all about Authorization and not Authentication
(hope you remember the Story of Authentication vs Authorization)
OAuth is for authorization between services
OAuth is an open standard for access delegation

(Remember it allows limited access as per the permission)
It is an open standard so it can be used by any services and so all of them can use a standard way to communicate with each other.
Now there are 2 versions OAuth 1.0 and OAuth 2.0.
OAuth 2.0 is the latest implementation and is widely and commonly used.
Minnie – Now I am getting this well.
Mickey – Now check this Wikipedia definition again
“OAuth is an open standard for access delegation, commonly used as a way for Internet users to grant websites or applications access to their information on other websites but without giving them the passwords”
Minnie – Yes, now this is making meaning
Mickey – Another way of saying this:
“OAuth is an open-standard authorization protocol or framework that describes how unrelated servers and services can safely allow authenticated access to their assets without actually sharing the login credential“
Mickey – I got it.
Mickey – Well, so shall we go and explain this to your friend?
Minnie – I am in no hurry now. Let’s go to the riverside and spend some time together.
Mickey – As you say, Ma’am. Let’s Go.
Minnie – Thank You! I feel so good now.
Mickey – I am always here for you 🙂

about the author  |  more stories


214
			
			
			
			
			
			Share this:TwitterFacebookGoogleSkypeLinkedIn

	Related
A Story of Automation FrameworkJuly 20, 2020In &quot;Mickey and Minnie Stories&quot;A Story of Artificial Intelligence (AI)April 26, 2020In &quot;Mickey and Minnie Stories&quot;A Story of JSONMay 4, 2020In &quot;Mickey and Minnie Stories&quot;
	 		Tags: explain oauth with example, how oauth works, oauth analogy, oauth for beginners, what is oauth
	
		

			

			
				
				
				
				
			

			
			
			

			
				
				
				
				
					
					
				
			

		
		
		
	

		


	 
	


	

    
    
    

Latest News   Stories by Raghav  -  click here                    |             Click here for Udemy Courses by Raghav           |           To Invite Raghav for Events or Corporate Training email - training.raghav@gmail.com      |            Stories by Raghav  -  click here                    |             Click here for Udemy Courses by Raghav           |           To Invite Raghav for Events or Corporate Training email - training.raghav@gmail.com      |         jQuery(function(){jQuery(&quot; , &quot;'&quot; , &quot;.marquee-hsas-widget-49&quot; , &quot;'&quot; , &quot;).marquee({allowCss3Support: true,css3easing: &quot; , &quot;'&quot; , &quot;linear&quot; , &quot;'&quot; , &quot;,easing: &quot; , &quot;'&quot; , &quot;linear&quot; , &quot;'&quot; , &quot;,delayBeforeStart: 2000,direction: &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;,duplicated: true,duration: 6000,gap: 100,pauseOnCycle: true,pauseOnHover: true,startVisible: true});});Learn something new step by stepJoin my YouTube family

Archives
Mickey &amp; Minnie Stories
Online Courses
Training by Raghav
Quiz Time
Udemy Courses
Selenium
Jenkins
JMeter
Docker
Git and GitHub
Java for Automation
Katalon Studio
Maven
Redis
REST Web Services (API)
SOAP Web Services (API)
Selenium Introduction
UI Testing
API Testing
Mobile Testing
CI CD DevOps
Performance Testing
Newsletter
YouTube Playlists
SoapUI MCQ Quiz
Katalon Studio MCQ Quiz
Free Online Tutorials
Groovy Quiz
Jenkins Foundation Training
Groovy Beginners Training
SoapUI Beginners Training
Postman Beginners Training
About
Newsletter
The Testing QUIZ
BDD QUIZ
Python Quiz
Cypress Quiz
Corporate Training
IntelliJ IDEA QUIZ
REST Assured QUIZ
Introduction to Mobile Testing &amp; Appium
Selenium Quiz
Demo Quiz 1
Gatling Quiz 1
Gatling Quiz
Gatling Quiz 2
Hindi Tutorials
Select Language​▼visitor count2,953,924 CategoriesCategories
	Select Category
	Author : Raghav Pal  (3)
	Automation  (166)
	CI  (22)
	DevOps  (23)
	Docker  (6)
	Interview Questions  (4)
	Java  (11)
	Jenkins  (16)
	JMeter  (38)
	JMeter Issues  (2)
	Mickey and Minnie Stories  (49)
	Performance Testing  (36)
	Python  (1)
	Selenium  (28)
	Stories  (2)
	Testing  (41)
	Tools  (5)



/* &lt;![CDATA[ */
(function() {
	var dropdown = document.getElementById( &quot;cat&quot; );
	function onCatChange() {
		if ( dropdown.options[ dropdown.selectedIndex ].value > 0 ) {
			dropdown.parentNode.submit();
		}
	}
	dropdown.onchange = onCatChange;
})();
/* ]]> */


			
	









	
		
		Archives
Mickey &amp; Minnie Stories
Online Courses
Training by Raghav
Quiz Time
Udemy Courses
Selenium
Jenkins
JMeter
Docker
Git and GitHub
Java for Automation
Katalon Studio
Maven
Redis
REST Web Services (API)
SOAP Web Services (API)
Selenium Introduction
UI Testing
API Testing
Mobile Testing
CI CD DevOps
Performance Testing
Newsletter
YouTube Playlists
SoapUI MCQ Quiz
Katalon Studio MCQ Quiz
Free Online Tutorials
Groovy Quiz
Jenkins Foundation Training
Groovy Beginners Training
SoapUI Beginners Training
Postman Beginners Training
About
Newsletter
The Testing QUIZ
BDD QUIZ
Python Quiz
Cypress Quiz
Corporate Training
IntelliJ IDEA QUIZ
REST Assured QUIZ
Introduction to Mobile Testing &amp; Appium
Selenium Quiz
Demo Quiz 1
Gatling Quiz 1
Gatling Quiz
Gatling Quiz 2
Hindi Tutorials
	
	
	    	
		
			
				Automation Step by Step © 2023				
				
							
		

		
			
				Theme by				WP Puzzle
			
						
		
	
    	



&quot;))]</value>
      <webElementGuid>1248f597-c303-4f64-9d85-e9336edc9579</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
