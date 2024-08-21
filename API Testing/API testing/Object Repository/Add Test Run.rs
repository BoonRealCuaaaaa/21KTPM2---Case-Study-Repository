<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Test Run</name>
   <tag></tag>
   <elementGuidId>eafcce2e-60e1-4e6b-9491-d2b1c4a26d36</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>20000</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;title\&quot;: \&quot;${title}\&quot;,\n    \&quot;description\&quot;: \&quot;${desc}\&quot;,\n    \&quot;environment\&quot;: \&quot;${env}\&quot;,\n    \&quot;runType\&quot;:\&quot;${type}\&quot;,\n    \&quot;tag\&quot;:\&quot;${tag}\&quot;,\n    \&quot;testCases\&quot;:${testcases}\n}&quot;,
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
      <webElementGuid>e47ce83b-854c-4ff9-85f2-9ab07c9409d7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookie}</value>
      <webElementGuid>c61ba47c-90c0-4538-b6bd-0e04c79a715d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://qa-fengine-prod.onrender.com/project/${projectId}/testrun/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>20000</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.cookie</defaultValue>
      <description></description>
      <id>e6fd6fdc-aee0-4e54-a84f-40c7764fc126</id>
      <masked>false</masked>
      <name>cookie</name>
   </variables>
   <variables>
      <defaultValue>['207']</defaultValue>
      <description></description>
      <id>36dac7ae-ac97-443a-a181-b43fe2a89306</id>
      <masked>false</masked>
      <name>testcases</name>
   </variables>
   <variables>
      <defaultValue>'Title'</defaultValue>
      <description></description>
      <id>c9a503a6-c52b-41de-bbe2-a09ae3fb0ac8</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>'Desc'</defaultValue>
      <description></description>
      <id>1513cf26-46e8-4b2f-b088-f827fe84d6f0</id>
      <masked>false</masked>
      <name>desc</name>
   </variables>
   <variables>
      <defaultValue>'not-set'</defaultValue>
      <description></description>
      <id>6f3ccf69-70ee-47af-ab8f-f52a964a7968</id>
      <masked>false</masked>
      <name>env</name>
   </variables>
   <variables>
      <defaultValue>'Manual'</defaultValue>
      <description></description>
      <id>e972ccb2-27eb-469a-90e5-5a3d3b1228af</id>
      <masked>false</masked>
      <name>tag</name>
   </variables>
   <variables>
      <defaultValue>'Manual'</defaultValue>
      <description></description>
      <id>6d711a80-c37d-40ff-a24c-ac3491208276</id>
      <masked>false</masked>
      <name>type</name>
   </variables>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>a4106acd-520b-4190-84f1-48ef0b0fa44b</id>
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
