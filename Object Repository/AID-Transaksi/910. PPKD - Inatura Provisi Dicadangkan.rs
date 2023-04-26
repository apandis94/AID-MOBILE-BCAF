<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>910. PPKD - Inatura Provisi Dicadangkan</name>
   <tag></tag>
   <elementGuidId>34ddd99e-0e27-40ee-b70e-46a76b76134d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;NoKontrak\&quot;: \&quot;${nokontrak}\&quot;,\n  \&quot;JenisCadangan\&quot;: \&quot;${jenis_cad}\&quot;,\n  \&quot;Persentase\&quot;: ${persentase},\n  \&quot;NominalKomisi\&quot;: ${amount_komisi}\n}\n&quot;,
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
      <webElementGuid>8034d52f-df95-4b3e-b175-d297ac2e74b0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>f0a98974-3fc5-4490-8364-9a281812f16a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/PermohonanPembayaranKomisiDealer/InaturaProvisi/Dicadangkan/NominalKomisi</restUrl>
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
      <id>cef689fb-69ec-47a4-bdf5-4845ef55488b</id>
      <masked>false</masked>
      <name>nokontrak</name>
   </variables>
   <variables>
      <defaultValue>'Cadangan Dana Program'</defaultValue>
      <description></description>
      <id>15e739f2-ad2d-4f31-9026-d7fdf41cccf3</id>
      <masked>false</masked>
      <name>jenis_cad</name>
   </variables>
   <variables>
      <defaultValue>4.0</defaultValue>
      <description></description>
      <id>baaaa6fd-96e7-419e-9b3c-1f73b9712b16</id>
      <masked>false</masked>
      <name>persentase</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>28580802-a1e7-43aa-ac2d-89b2f1cd9682</id>
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
