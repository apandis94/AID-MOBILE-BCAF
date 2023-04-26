<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>914. PPKD - Komisi Admin Dibagikan</name>
   <tag></tag>
   <elementGuidId>2c7d86ba-2da7-4f36-95df-5ce1e21504d1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${nokontrak}\&quot;,\n  \&quot;IDPenerima\&quot;: ${id_penerima}, // diambil dari Detail daftar penerima\n  \&quot;AccountNo\&quot;: \&quot;${no_rekening}\&quot;, // diambil dari Detail daftar penerima\n  \&quot;Persentase\&quot;: ${persentase},\n  \&quot;NominalKomisi\&quot;: ${amount}\n}\n&quot;,
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
      <webElementGuid>8e4b491d-0bd3-4e92-a7ea-bbee1fbc7fc8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>6e87a666-9868-43b1-b740-1268b6a9de47</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/KomisiAdmin/Dibagikan/NominalKomisi</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'9530708518-001'</defaultValue>
      <description></description>
      <id>9b5cee28-b4be-4a4f-b6d2-53d3ac6ac582</id>
      <masked>false</masked>
      <name>nokontrak</name>
   </variables>
   <variables>
      <defaultValue>1003</defaultValue>
      <description></description>
      <id>b93e2ddb-6de1-4d25-ac5d-6ab50bae2f78</id>
      <masked>false</masked>
      <name>id_penerima</name>
   </variables>
   <variables>
      <defaultValue>'1340420175'</defaultValue>
      <description></description>
      <id>acd632e2-e151-4aef-ad34-0b73cdbaa155</id>
      <masked>false</masked>
      <name>no_rekening</name>
   </variables>
   <variables>
      <defaultValue>4.0</defaultValue>
      <description></description>
      <id>50e21a84-6461-4562-bd61-d82e30b6e61e</id>
      <masked>false</masked>
      <name>persentase</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>87accc67-8f74-4bdb-96fd-2886ccf958b4</id>
      <masked>false</masked>
      <name>amount</name>
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
