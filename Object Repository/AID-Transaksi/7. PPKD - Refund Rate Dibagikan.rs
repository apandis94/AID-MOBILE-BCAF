<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>7. PPKD - Refund Rate Dibagikan</name>
   <tag></tag>
   <elementGuidId>1213619d-b632-44ab-8f78-748f3fa9713b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${accno}\&quot;,\n  \&quot;IDPenerima\&quot;: ${id_penerima}, // diambil dari Detail daftar penerima\n  \&quot;AccountNo\&quot;: \&quot;${norekening}\&quot;, // diambil dari Detail daftar penerima\n  \&quot;Persentase\&quot;: ${refundrate_percent},\n  \&quot;SukuBunga\&quot;: ${sukubunga},\n  \&quot;NominalKomisi\&quot;: ${amount_komisi}\n}\n&quot;,
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
      <webElementGuid>9466ef84-1659-41e9-903e-6fe82638bf00</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>7e1df430-4bb2-4a9d-b8d2-fe526c65df94</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/RefundRate/Dibagikan/NominalKomisi</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'9490713559-001'</defaultValue>
      <description></description>
      <id>36f51301-e448-4b08-827d-d94f6cab246a</id>
      <masked>false</masked>
      <name>accno</name>
   </variables>
   <variables>
      <defaultValue>41917</defaultValue>
      <description></description>
      <id>ee3460fc-b637-45f7-83e6-fcae5db7533f</id>
      <masked>false</masked>
      <name>id_penerima</name>
   </variables>
   <variables>
      <defaultValue>'1349077777'</defaultValue>
      <description></description>
      <id>af3c05c6-366f-4866-a5ab-880041e1ee29</id>
      <masked>false</masked>
      <name>norekening</name>
   </variables>
   <variables>
      <defaultValue>5.5045</defaultValue>
      <description></description>
      <id>62134646-323a-4b19-a7a5-b6228e455659</id>
      <masked>false</masked>
      <name>refundrate_percent</name>
   </variables>
   <variables>
      <defaultValue>11</defaultValue>
      <description></description>
      <id>3896a387-3203-4977-bca0-100a31c5c097</id>
      <masked>false</masked>
      <name>sukubunga</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>9c28c2c8-d5a2-4b6c-a1bc-c8e39b97aac6</id>
      <masked>false</masked>
      <name>amount_komisi</name>
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
