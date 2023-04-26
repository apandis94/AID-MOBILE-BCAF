<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>913. PPKD - Inatura Program Dicadangkan</name>
   <tag></tag>
   <elementGuidId>4d873560-5f44-4237-a086-7d8df2221bfe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${nokontrak}\&quot;,\n  \&quot;JenisCadangan\&quot;: \&quot;${jenis_cad}\&quot;,\n  \&quot;InaturaProgramID\&quot;: ${id_inatura_program}, // Diambil Dari Master Inatura Program\n  \&quot;Persentase\&quot;: ${persentase},\n  \&quot;NominalKomisi\&quot;: ${amount_komisi}\n}\n&quot;,
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
      <webElementGuid>d9020894-31ec-426f-9c73-a336ece5ffbb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>b2f033b4-4a4f-4f26-a558-f09bfa5f7119</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/InaturaProgram/Dicadangkan/NominalKomisi</restUrl>
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
      <id>c4e034ac-eb59-421c-9a8f-f2a9cfe991c2</id>
      <masked>false</masked>
      <name>nokontrak</name>
   </variables>
   <variables>
      <defaultValue>'Cadangan Dana Program'</defaultValue>
      <description></description>
      <id>c69ddcfc-cbb3-4380-9553-5cc2005fb4ff</id>
      <masked>false</masked>
      <name>jenis_cad</name>
   </variables>
   <variables>
      <defaultValue>4.0</defaultValue>
      <description></description>
      <id>5dd72b12-33a7-46e3-a2ba-8db0317c2308</id>
      <masked>false</masked>
      <name>persentase</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>21e74f6a-e1e7-4fa2-964e-74e85b5a3567</id>
      <masked>false</masked>
      <name>amount_komisi</name>
   </variables>
   <variables>
      <defaultValue>3047</defaultValue>
      <description></description>
      <id>9b6456d5-cca1-41b5-87c8-c494f60bc6a8</id>
      <masked>false</masked>
      <name>id_inatura_program</name>
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
