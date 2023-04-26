<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>2. PPKD - Get Realisasi</name>
   <tag></tag>
   <elementGuidId>99fe5ae3-b1ad-4d39-a99f-80b328ce6911</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoRek\&quot;: \&quot;${norek}\&quot;,\n  \&quot;NoPIN\&quot;: \&quot;${nopin}\&quot;\n}\n\n//Norek Nopin diambil dari respons PPKD- Search Realisasi (Nomor kontrak -\u003e xxxxxxxx-xxx)&quot;,
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
      <webElementGuid>f7c70d98-a709-4e95-9060-e873dbd8421b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>e906ea0f-a7fc-425e-8d12-e9939d463da5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/GetRealisasi</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'9490713559'</defaultValue>
      <description></description>
      <id>315f9b66-69d3-4a3f-8f20-611bdc809ccf</id>
      <masked>false</masked>
      <name>norek</name>
   </variables>
   <variables>
      <defaultValue>'001'</defaultValue>
      <description></description>
      <id>9cc55089-5993-4dde-b991-860831cc82ec</id>
      <masked>false</masked>
      <name>nopin</name>
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
