<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add defect</name>
   <tag></tag>
   <elementGuidId>fc8b4f2c-4f3a-405b-b698-5286eb42accb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;title\&quot;:\&quot;${title}\&quot;,\&quot;description\&quot;:\&quot;${desc}\&quot;,\&quot;assignee\&quot;:\&quot;${assignee}\&quot;,\&quot;severity\&quot;:\&quot;${severity}\&quot;,\&quot;priority\&quot;:\&quot;${priority}\&quot;,\&quot;environment\&quot;:\&quot;${env}\&quot;,\&quot;step\&quot;:\&quot;${step}\&quot;}&quot;,
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
      <webElementGuid>a3be2da1-4560-4974-882e-be5bbacf38c1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookie}</value>
      <webElementGuid>2b03bb77-f48d-47d9-a093-18d2baf97979</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://qa-fengine-prod.onrender.com/project/${projectId}/defect/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Title'</defaultValue>
      <description></description>
      <id>af975bc3-bf3d-437a-9092-1279ecf261b5</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>'Desc'</defaultValue>
      <description></description>
      <id>e3f35d6c-7eb6-42aa-895a-fcdfb8fcd121</id>
      <masked>false</masked>
      <name>desc</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>d41036bc-003a-40c9-84da-c8698101883a</id>
      <masked>false</masked>
      <name>assignee</name>
   </variables>
   <variables>
      <defaultValue>'Normal'</defaultValue>
      <description></description>
      <id>cf99b19b-b74c-4820-8c03-339a6b2e2ab7</id>
      <masked>false</masked>
      <name>severity</name>
   </variables>
   <variables>
      <defaultValue>'Highest'</defaultValue>
      <description></description>
      <id>dbf39059-36c5-4fe4-b23f-ab10b28754ef</id>
      <masked>false</masked>
      <name>priority</name>
   </variables>
   <variables>
      <defaultValue>'Production'</defaultValue>
      <description></description>
      <id>be12a87f-4332-414b-8d7e-6ef383b8b1b7</id>
      <masked>false</masked>
      <name>env</name>
   </variables>
   <variables>
      <defaultValue>'1. Do nothing'</defaultValue>
      <description></description>
      <id>16c394e4-b834-4ae5-877c-0ec883b01237</id>
      <masked>false</masked>
      <name>step</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookie</defaultValue>
      <description></description>
      <id>b42971f9-2606-4da9-a8cb-82c6b068feca</id>
      <masked>false</masked>
      <name>cookie</name>
   </variables>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>390ae623-542f-4601-abd5-ccbd78c41a31</id>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
