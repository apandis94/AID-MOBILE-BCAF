<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>912. PPKD - Inatura Program Dibagikan</name>
   <tag></tag>
   <elementGuidId>d2256a2c-ab58-41bc-bc90-d53c895ec25f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${accno}\&quot;,\n  \&quot;IDPenerima\&quot;: ${idpenerima}, // diambil dari Detail daftar penerima\n  \&quot;AccountNo\&quot;: \&quot;${norek}\&quot;, // diambil dari Detail daftar penerima\n  \&quot;InaturaProgramID\&quot;: ${inatura_program}, // Diambil Dari Master Inatura Program\n  \&quot;NominalKomisi\&quot;: ${amnt_komisi}\n}\n&quot;,
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
      <webElementGuid>443cfe87-7f09-42a2-a926-c0f5e9cbb6a0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>ee324f3b-07c8-4491-941c-59b2ec40b90a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/InaturaProgram/Dibagikan/NominalKomisi</restUrl>
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
      <id>f38f7ca7-1bb6-47ae-9282-111453587c95</id>
      <masked>false</masked>
      <name>accno</name>
   </variables>
   <variables>
      <defaultValue>1003</defaultValue>
      <description></description>
      <id>50e72544-c133-47ea-a21a-7a8666a0ce9d</id>
      <masked>false</masked>
      <name>idpenerima</name>
   </variables>
   <variables>
      <defaultValue>'1340420175'</defaultValue>
      <description></description>
      <id>88b5f664-07f9-415a-b4da-a4b6dc84ad2f</id>
      <masked>false</masked>
      <name>norek</name>
   </variables>
   <variables>
      <defaultValue>'3047'</defaultValue>
      <description></description>
      <id>7248c143-84a4-4c6a-9ea4-07c0b44c6a26</id>
      <masked>false</masked>
      <name>inatura_program</name>
   </variables>
   <variables>
      <defaultValue>100000</defaultValue>
      <description></description>
      <id>c00cf3ce-6768-410d-8408-4b4ef87d09e4</id>
      <masked>false</masked>
      <name>amnt_komisi</name>
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
