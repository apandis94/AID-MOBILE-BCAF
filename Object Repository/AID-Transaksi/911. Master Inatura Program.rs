<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>911. Master Inatura Program</name>
   <tag></tag>
   <elementGuidId>2d4ec554-f07f-44bf-bb59-add831fca06c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;PackageId\&quot;: \&quot;${id}\&quot;,\n  \&quot;ProductId\&quot;: \&quot;${productid}\&quot;,\n  \&quot;KondisiMobil\&quot;: \&quot;${car_condition}\&quot;,\n  \&quot;TglProceedPrinting\&quot;: \&quot;${tgl_printing}\&quot;,\n  \&quot;TglRealisasi\&quot;: \&quot;${tgl_real}\&quot;\n}&quot;,
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
      <webElementGuid>c40f04a9-c296-481e-aba1-180290d4e97f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>b62b0e66-ea4e-4bc9-b4f6-7d87ca1f8ec9</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/MasterInaturaProgram</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'000'</defaultValue>
      <description></description>
      <id>17f973d2-e998-4fc6-8935-5791fc5c80c7</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'N'</defaultValue>
      <description></description>
      <id>b950a318-7356-4f65-97c2-410f25d7b22f</id>
      <masked>false</masked>
      <name>productid</name>
   </variables>
   <variables>
      <defaultValue>'New Vehicle'</defaultValue>
      <description></description>
      <id>ff00f57d-7c7f-4db4-aeac-2c7162206a86</id>
      <masked>false</masked>
      <name>car_condition</name>
   </variables>
   <variables>
      <defaultValue>'20220203'</defaultValue>
      <description></description>
      <id>d7e6964c-6e15-4d99-9f1c-9b2898b1e59a</id>
      <masked>false</masked>
      <name>tgl_printing</name>
   </variables>
   <variables>
      <defaultValue>'20220203'</defaultValue>
      <description></description>
      <id>354e0267-7e1a-4cfa-a5d7-359d9e00cf51</id>
      <masked>false</masked>
      <name>tgl_real</name>
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
