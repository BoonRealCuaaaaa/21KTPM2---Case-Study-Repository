<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add test case</name>
   <tag></tag>
   <elementGuidId>a86bc1f9-8a51-44b4-8a62-b287a3cb7aaf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;title\&quot;:\&quot;${title}\&quot;,\n    \&quot;status\&quot;:\&quot;${status}\&quot;,\n    \&quot;priority\&quot;:\&quot;${priority}\&quot;,\n    \&quot;severity\&quot;:\&quot;${severity}\&quot;,\n    \&quot;description\&quot;:\&quot;${desc}\&quot;,\n    \&quot;suite\&quot;:\&quot;${suite}\&quot;,\n    \&quot;preConditions\&quot;:\&quot;${pre}\&quot;,\n    \&quot;postConditions\&quot;:\&quot;${post}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0b4db5c7-569e-4582-bb6f-b4148780d572</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookie}</value>
      <webElementGuid>5f5baa68-925c-46e2-a220-6c3be654da2a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://qa-fengine-prod.onrender.com/project/${projectId}/test-case/create-test-case</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.cookie</defaultValue>
      <description></description>
      <id>47786262-6d03-4551-baf1-db341fe990f0</id>
      <masked>false</masked>
      <name>cookie</name>
   </variables>
   <variables>
      <defaultValue>'Title'</defaultValue>
      <description></description>
      <id>c6db5b5b-4762-4879-9c6f-86041ccf53ab</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>'Actual'</defaultValue>
      <description></description>
      <id>c0240eee-d0dc-4f94-a7b6-ef47d17c5312</id>
      <masked>false</masked>
      <name>status</name>
   </variables>
   <variables>
      <defaultValue>'High'</defaultValue>
      <description></description>
      <id>824bf318-8fa0-4568-ac0b-b45b53f21b7f</id>
      <masked>false</masked>
      <name>priority</name>
   </variables>
   <variables>
      <defaultValue>'Normal'</defaultValue>
      <description></description>
      <id>3809a9a1-802f-4e5b-bb96-04d0a2f70e45</id>
      <masked>false</masked>
      <name>severity</name>
   </variables>
   <variables>
      <defaultValue>'Description'</defaultValue>
      <description></description>
      <id>683e64f9-b0c2-4d0c-b2de-8d4a348c1ba8</id>
      <masked>false</masked>
      <name>desc</name>
   </variables>
   <variables>
      <defaultValue>-1</defaultValue>
      <description></description>
      <id>9abd0128-1270-4f91-9e93-8c755521940b</id>
      <masked>false</masked>
      <name>suite</name>
   </variables>
   <variables>
      <defaultValue>'Precondition'</defaultValue>
      <description></description>
      <id>c77b246b-b971-4469-9f2d-b847f1034271</id>
      <masked>false</masked>
      <name>pre</name>
   </variables>
   <variables>
      <defaultValue>'Postcondition'</defaultValue>
      <description></description>
      <id>e59ec80f-f211-4fc2-b1b4-70fa264f6daf</id>
      <masked>false</masked>
      <name>post</name>
   </variables>
   <variables>
      <defaultValue>'1. Do nothing'</defaultValue>
      <description></description>
      <id>37636b2d-0b45-4c1e-8fc2-7a7b32080041</id>
      <masked>false</masked>
      <name>step</name>
   </variables>
   <variables>
      <defaultValue>'Do nothing'</defaultValue>
      <description></description>
      <id>330407e4-c98c-493e-9cf8-7bee0c319538</id>
      <masked>false</masked>
      <name>expected</name>
   </variables>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>33c3852b-cf6c-4172-8f01-69254fadab93</id>
      <masked>false</masked>
      <name>projectId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
