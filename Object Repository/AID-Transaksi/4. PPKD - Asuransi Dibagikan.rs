<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>4. PPKD - Asuransi Dibagikan</name>
   <tag></tag>
   <elementGuidId>5e3f9749-eaa5-4c77-a240-bbf77347a2c3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${accno}\&quot;,\n  \&quot;IDPenerima\&quot;: ${id}, // diambil dari Detail daftar penerima\n  \&quot;AccountNo\&quot;: \&quot;${no_rekening}\&quot;, // diambil dari Detail daftar penerima\n  \&quot;Persentase\&quot;: ${percent},\n  \&quot;NominalKomisi\&quot;: ${komisi}\n}\n&quot;,
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
      <webElementGuid>c632feb3-6ab3-47bb-b845-7172e9e6c3ad</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>3693ef03-d08c-4893-80b6-b4c0bd1d658d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/KomisiAsuransi/Dibagikan/NominalKomisi</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'9530708518-001'</defaultValue>
      <description></description>
      <id>c0c20544-4c2b-4219-bda8-c262c3e1a5f3</id>
      <masked>false</masked>
      <name>accno</name>
   </variables>
   <variables>
      <defaultValue>1003</defaultValue>
      <description></description>
      <id>15b402e6-c6ef-4ffc-806a-256b5b572714</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'1340420175'</defaultValue>
      <description></description>
      <id>a8b024d0-288b-4d7e-9ccd-4ae8c1e5d94e</id>
      <masked>false</masked>
      <name>no_rekening</name>
   </variables>
   <variables>
      <defaultValue>25</defaultValue>
      <description></description>
      <id>d1b6d14c-8839-4ece-a7cc-22a7083dd043</id>
      <masked>false</masked>
      <name>percent</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>f4788e7f-29f0-41c0-b52a-6f1c58281bb9</id>
      <masked>false</masked>
      <name>komisi</name>
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
